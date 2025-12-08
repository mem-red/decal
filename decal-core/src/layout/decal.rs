use crate::layout::{FontRegistry, Node, NodeKind};
use crate::layout::{NodeId, VectorizationError};
use crate::prelude::Typography;
use resvg::render;
use smallvec::SmallVec;
use std::fmt::Write;
use std::sync::{Arc, Mutex};
use taffy::prelude::TaffyMaxContent;
use taffy::{
    CacheTree, LayoutPartialTree, PrintTree, RoundTree, TraversePartialTree, TraverseTree,
    compute_block_layout, compute_cached_layout, compute_flexbox_layout, compute_grid_layout,
    compute_leaf_layout, compute_root_layout, print_tree, round_layout,
};
use thiserror::Error;
use tiny_skia::{Pixmap, Transform};
use usvg::{Options, Tree};

const ROOT_ID: usize = 0;
const INLINE_FRAG_CASCADE: usize = 16;

#[derive(Debug, Error)]
pub enum RasterizationError {
    #[error("vectorization error")]
    Vectorization(#[from] VectorizationError),
    #[error("cannot write to stream")]
    SvgWrite(#[from] std::fmt::Error),
    #[error("svg parsing error")]
    SvgParse(#[from] usvg::Error),
    #[error("pixmap alloc error")]
    PixmapAlloc,
}

#[derive(Debug)]
pub struct Decal {
    fonts: Arc<Mutex<FontRegistry>>,
    nodes: Vec<Node>,
}

impl Decal {
    pub fn new(root: Node) -> Self {
        Self {
            nodes: vec![root],
            fonts: Arc::new(Mutex::new(FontRegistry::new())),
        }
    }

    pub fn root_id(&self) -> NodeId {
        ROOT_ID.into()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn append_child(&mut self, parent_id: NodeId, mut child: Node) -> NodeId {
        self.assert_non_atomic(parent_id);
        let parent = &self.nodes[parent_id];
        child.typography.cascade_from(&parent.typography);

        // copy the resolved typography into its metadata
        if let NodeKind::Text(ref mut meta) = child.kind {
            meta.set_typography(child.typography.clone());
        }

        self.nodes.push(child);
        let child_id = self.nodes.len() - 1;
        self.nodes[parent_id].children.push(child_id);
        child_id.into()
    }

    pub fn append_fragment(&mut self, parent_id: NodeId, mut fragment: Decal) {
        if fragment.nodes.is_empty() {
            return;
        }

        self.assert_non_atomic(parent_id);

        let parent_typography = &self.nodes[parent_id].typography;
        Self::cascade_typography_subtree(&mut fragment.nodes, parent_typography);

        let root_id = self.nodes.len(); // fragment root node
        self.nodes.reserve(fragment.nodes.len()); // pre-allocation
        self.nodes[parent_id].children.push(root_id);

        for mut node in fragment.nodes {
            // update child indices after adding them to main arena
            for child_id in node.children.iter_mut() {
                *child_id += root_id;
            }

            self.nodes.push(node);
        }
    }

    #[allow(dead_code)]
    pub(crate) fn print_tree(&self) {
        print_tree(self, taffy::NodeId::from(ROOT_ID));
    }

    pub(crate) fn rasterize(
        &self,
        options: Option<Options>,
        transform: Option<Transform>,
    ) -> Result<Pixmap, RasterizationError> {
        let tree = Tree::from_str(&self.vectorize()?, &(options.unwrap_or_default()))
            .map_err(RasterizationError::SvgParse)?;
        let mut pixmap = Pixmap::new(tree.size().width() as u32, tree.size().height() as u32)
            .ok_or(RasterizationError::PixmapAlloc)?;
        render(&tree, transform.unwrap_or_default(), &mut pixmap.as_mut());
        Ok(pixmap)
    }

    pub(crate) fn vectorize(&self) -> Result<String, VectorizationError> {
        let mut out = String::new();
        let root = &self.nodes[ROOT_ID];

        if let NodeKind::Root(meta) = &root.kind {
            write!(
                out,
                r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#,
                meta.width, meta.height,
            )?;
        }

        self.write_node(
            &mut out,
            (root.final_layout.size.width, root.final_layout.size.height),
            taffy::NodeId::from(ROOT_ID),
        )?;

        out.push_str("</svg>");

        Ok(out)
    }

    pub(crate) fn compute_layout(&mut self) {
        compute_root_layout(self, taffy::NodeId::from(ROOT_ID), taffy::Size::MAX_CONTENT);
        round_layout(self, taffy::NodeId::from(ROOT_ID));
    }

    fn cascade_typography_subtree(nodes: &mut [Node], parent_typography: &Typography) {
        let mut stack: SmallVec<[(usize, Typography); INLINE_FRAG_CASCADE]> = SmallVec::new();
        stack.push((ROOT_ID, parent_typography.clone()));

        while let Some((idx, mut parent)) = stack.pop() {
            let node = &mut nodes[idx];
            node.typography.cascade_from(&parent);

            if let NodeKind::Text(ref mut meta) = node.kind {
                meta.set_typography(node.typography.clone());
            }

            if !node.children.is_empty() {
                parent = node.typography.clone();
            }

            for &child in &node.children {
                stack.push((child, parent.clone()));
            }
        }
    }

    /// Panics if the node with the given `id` is atomic (cannot have children).
    ///
    /// Note: This is a safety check. The macro should prevent adding children to atomic nodes at compile time.
    fn assert_non_atomic(&self, id: NodeId) {
        if self.nodes[id].kind.is_atomic() {
            panic!("node with id {id} is atomic and cannot contain children");
        }
    }

    pub(crate) fn set_fonts(&mut self, fonts: Arc<Mutex<FontRegistry>>) {
        self.fonts = fonts;
    }

    #[inline(always)]
    fn node_from_id(&self, node_id: taffy::NodeId) -> &Node {
        &self.nodes[usize::from(node_id)]
    }

    #[inline(always)]
    fn node_from_id_mut(&mut self, node_id: taffy::NodeId) -> &mut Node {
        &mut self.nodes[usize::from(node_id)]
    }

    fn write_node<T>(
        &self,
        out: &mut T,
        root_size: (f32, f32),
        node_id: taffy::NodeId,
    ) -> Result<(), VectorizationError>
    where
        T: Write,
    {
        let node_idx = usize::from(node_id);
        let node = &self.nodes[node_idx];

        if node.visual.visible && !matches!(node.layout.display, taffy::Display::None) {
            node.write_svg_start(out, root_size, self.fonts.clone(), node_idx)?;

            for &child_id in &node.children {
                self.write_node(out, root_size, taffy::NodeId::from(child_id))?;
            }

            node.write_svg_end(out, root_size, node_idx)?;
        }

        Ok(())
    }
}

impl std::ops::Index<NodeId> for Vec<Node> {
    type Output = Node;
    fn index(&self, id: NodeId) -> &Self::Output {
        &self[usize::from(id)]
    }
}

impl std::ops::IndexMut<NodeId> for Vec<Node> {
    fn index_mut(&mut self, id: NodeId) -> &mut Self::Output {
        &mut self[usize::from(id)]
    }
}

pub struct ChildIter<'a>(std::slice::Iter<'a, usize>);

impl Iterator for ChildIter<'_> {
    type Item = taffy::NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied().map(taffy::NodeId::from)
    }
}

impl TraversePartialTree for Decal {
    type ChildIter<'a> = ChildIter<'a>;

    fn child_ids(&self, node_id: taffy::NodeId) -> Self::ChildIter<'_> {
        ChildIter(self.node_from_id(node_id).children.iter())
    }

    fn child_count(&self, node_id: taffy::NodeId) -> usize {
        self.node_from_id(node_id).children.len()
    }

    fn get_child_id(&self, node_id: taffy::NodeId, index: usize) -> taffy::NodeId {
        taffy::NodeId::from(self.node_from_id(node_id).children[index])
    }
}

impl TraverseTree for Decal {}

impl LayoutPartialTree for Decal {
    type CoreContainerStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;

    type CustomIdent = String;

    fn get_core_container_style(&self, node_id: taffy::NodeId) -> Self::CoreContainerStyle<'_> {
        &self.node_from_id(node_id).layout
    }

    fn resolve_calc_value(&self, _val: *const (), _basis: f32) -> f32 {
        0.0
    }

    fn set_unrounded_layout(&mut self, node_id: taffy::NodeId, layout: &taffy::Layout) {
        self.node_from_id_mut(node_id).unrounded_layout = *layout;
    }

    fn compute_child_layout(
        &mut self,
        node_id: taffy::NodeId,
        inputs: taffy::tree::LayoutInput,
    ) -> taffy::tree::LayoutOutput {
        compute_cached_layout(self, node_id, inputs, |tree, node_id, inputs| {
            let node = &mut tree.nodes[usize::from(node_id)];
            match node.kind {
                NodeKind::Root(_) | NodeKind::Block => compute_block_layout(tree, node_id, inputs),
                NodeKind::Flex | NodeKind::Column | NodeKind::Row => {
                    compute_flexbox_layout(tree, node_id, inputs)
                }
                NodeKind::Grid => compute_grid_layout(tree, node_id, inputs),
                NodeKind::Text(ref mut meta) => compute_leaf_layout(
                    inputs,
                    &node.layout,
                    |_val, _basis| 0.0,
                    |known_dimensions, available_space| {
                        meta.measure(known_dimensions, available_space, tree.fonts.clone())
                    },
                ),
                NodeKind::Image(ref mut meta) => compute_leaf_layout(
                    inputs,
                    &node.layout,
                    |_val, _basis| 0.0,
                    |known_dimensions, _available_space| meta.measure(known_dimensions),
                ),
            }
        })
    }
}

impl CacheTree for Decal {
    fn cache_get(
        &self,
        node_id: taffy::NodeId,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
        run_mode: taffy::RunMode,
    ) -> Option<taffy::LayoutOutput> {
        self.node_from_id(node_id)
            .cache
            .get(known_dimensions, available_space, run_mode)
    }

    fn cache_store(
        &mut self,
        node_id: taffy::NodeId,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
        run_mode: taffy::RunMode,
        layout_output: taffy::LayoutOutput,
    ) {
        self.node_from_id_mut(node_id).cache.store(
            known_dimensions,
            available_space,
            run_mode,
            layout_output,
        )
    }

    fn cache_clear(&mut self, node_id: taffy::NodeId) {
        self.node_from_id_mut(node_id).cache.clear();
    }
}

impl taffy::LayoutBlockContainer for Decal {
    type BlockContainerStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;

    type BlockItemStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;

    fn get_block_container_style(&self, node_id: taffy::NodeId) -> Self::BlockContainerStyle<'_> {
        &self.node_from_id(node_id).layout
    }

    fn get_block_child_style(&self, child_node_id: taffy::NodeId) -> Self::BlockItemStyle<'_> {
        &self.node_from_id(child_node_id).layout
    }
}

impl taffy::LayoutFlexboxContainer for Decal {
    type FlexboxContainerStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;

    type FlexboxItemStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;

    fn get_flexbox_container_style(
        &self,
        node_id: taffy::NodeId,
    ) -> Self::FlexboxContainerStyle<'_> {
        &self.node_from_id(node_id).layout
    }

    fn get_flexbox_child_style(&self, child_node_id: taffy::NodeId) -> Self::FlexboxItemStyle<'_> {
        &self.node_from_id(child_node_id).layout
    }
}

impl taffy::LayoutGridContainer for Decal {
    type GridContainerStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;

    type GridItemStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;

    fn get_grid_container_style(&self, node_id: taffy::NodeId) -> Self::GridContainerStyle<'_> {
        &self.node_from_id(node_id).layout
    }

    fn get_grid_child_style(&self, child_node_id: taffy::NodeId) -> Self::GridItemStyle<'_> {
        &self.node_from_id(child_node_id).layout
    }
}

impl RoundTree for Decal {
    fn get_unrounded_layout(&self, node_id: taffy::NodeId) -> taffy::Layout {
        self.node_from_id(node_id).unrounded_layout
    }

    fn set_final_layout(&mut self, node_id: taffy::NodeId, layout: &taffy::Layout) {
        let node = self.node_from_id_mut(node_id);
        node.final_layout = *layout;
        node.apply_layout_effects();
    }
}

impl PrintTree for Decal {
    fn get_debug_label(&self, node_id: taffy::NodeId) -> &'static str {
        match self.node_from_id(node_id).kind {
            NodeKind::Root(_) => "ROOT",
            NodeKind::Block => "BLOCK",
            NodeKind::Flex => "FLEX",
            NodeKind::Column => "COLUMN",
            NodeKind::Row => "ROW",
            NodeKind::Grid => "GRID",
            NodeKind::Text(_) => "TEXT",
            NodeKind::Image(_) => "IMAGE",
        }
    }

    fn get_final_layout(&self, node_id: taffy::NodeId) -> taffy::Layout {
        self.node_from_id(node_id).final_layout
    }
}
