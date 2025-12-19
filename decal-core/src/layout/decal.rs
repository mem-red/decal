use crate::layout::Typography;
use crate::layout::{FontRegistry, ImageCache, Node, NodeKind};
use crate::layout::{NodeId, VectorizationError};
use crate::paint::Resources;
use resvg::render;
use smallvec::SmallVec;
use std::fmt::Write;
use std::io::Read;
use std::sync::{Arc, Mutex};
use taffy::prelude::TaffyMaxContent;
use taffy::{
    CacheTree, LayoutPartialTree, PrintTree, RoundTree, TraversePartialTree, TraverseTree,
    compute_block_layout, compute_cached_layout, compute_flexbox_layout, compute_grid_layout,
    compute_leaf_layout, compute_root_layout, print_tree, round_layout,
};
use thiserror::Error;
use tiny_skia::{Pixmap, Transform};
use usvg::{ImageHrefResolver, ImageKind, Options, Tree};

const ROOT_ID: usize = 0;
const INLINE_FRAG_CASCADE: usize = 16;

#[derive(Debug, Error)]
pub enum RasterizationError {
    #[error("cannot rasterize a fragment")]
    Fragment,
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
    resources: Resources,
    nodes: Vec<Node>,
    is_fragment: bool,
}

impl Decal {
    pub fn new(root: Node, is_fragment: bool) -> Self {
        Self {
            fonts: Arc::new(Mutex::new(FontRegistry::new())),
            resources: Default::default(),
            nodes: vec![root],
            is_fragment,
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

        // register resources
        for resource in &child.resources {
            self.resources.get_or_add_resource(*resource);
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
        cascade_typography_subtree(&mut fragment.nodes, parent_typography);

        let root_id = self.nodes.len(); // fragment root node
        self.nodes.reserve(fragment.nodes.len()); // pre-allocation
        self.nodes[parent_id].children.push(root_id);

        for mut node in fragment.nodes {
            // update child indices after adding them to main arena
            for child_id in node.children.iter_mut() {
                *child_id += root_id;
            }

            // register resources
            for resource in &node.resources {
                self.resources.get_or_add_resource(*resource);
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
        image_cache: &ImageCache,
        options: Option<Options>,
        transform: Option<Transform>,
        debug: bool,
    ) -> Result<Pixmap, RasterizationError> {
        if self.is_fragment {
            return Err(RasterizationError::Fragment);
        }

        let tf = transform.unwrap_or_default();
        let mut options = options.unwrap_or_default();

        options.image_href_resolver = ImageHrefResolver {
            resolve_string: Box::new(move |href: &str, _opts: &Options| {
                let bytes = fetch_image_cached(image_cache, href)?;
                let kind = infer::get(&bytes)?;

                Some(match kind.mime_type() {
                    "image/png" => ImageKind::PNG(bytes.clone()),
                    "image/jpeg" => ImageKind::JPEG(bytes.clone()),
                    "image/webp" => ImageKind::WEBP(bytes.clone()),
                    "image/gif" => ImageKind::GIF(bytes.clone()),
                    _ => return None,
                })
            }),
            ..Default::default()
        };

        let tree =
            Tree::from_str(&self.vectorize()?, &options).map_err(RasterizationError::SvgParse)?;
        let size = tree.size();
        let mut pixmap = Pixmap::new(size.width() as u32, size.height() as u32)
            .ok_or(RasterizationError::PixmapAlloc)?;

        render(&tree, tf, &mut pixmap.as_mut());

        if debug {
            let mut bboxes = Vec::new();
            let mut stroke_bboxes = Vec::new();

            collect_bboxes(tree.root(), &mut bboxes, &mut stroke_bboxes);

            let stroke = tiny_skia::Stroke::default();
            let mut paint = tiny_skia::Paint::default();
            paint.set_color_rgba8(224, 16, 0, 195);

            for bbox in bboxes {
                let path = tiny_skia::PathBuilder::from_rect(bbox);
                pixmap.stroke_path(&path, &paint, &stroke, tf, None);
            }

            paint.set_color_rgba8(0, 45, 255, 127);

            for bbox in stroke_bboxes {
                let path = tiny_skia::PathBuilder::from_rect(bbox);
                pixmap.stroke_path(&path, &paint, &stroke, tf, None);
            }
        }

        Ok(pixmap)
    }

    pub(crate) fn vectorize(&self) -> Result<String, VectorizationError> {
        if self.is_fragment {
            return Err(VectorizationError::Fragment);
        }

        let mut out = String::new();
        let root = &self.nodes[ROOT_ID];

        if let NodeKind::Root(meta) = &root.kind {
            write!(
                out,
                r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#,
                meta.width, meta.height,
            )?;

            if !self.resources.is_empty() {
                write!(out, r#"<defs>{}</defs>"#, self.resources)?;
            }
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

    pub(crate) fn set_fonts(&mut self, fonts: Arc<Mutex<FontRegistry>>) {
        self.fonts = fonts;
    }

    /// Panics if the node with the given `id` is atomic (cannot have children).
    ///
    /// Note: This is a safety check. The macro should prevent adding children to atomic nodes at compile time.
    fn assert_non_atomic(&self, id: NodeId) {
        if self.nodes[id].kind.is_atomic() {
            panic!("node with id {id} is atomic and cannot contain children");
        }
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

//noinspection HttpUrlsUsage
fn fetch_image_cached(image_cache: &ImageCache, href: &str) -> Option<Arc<Vec<u8>>> {
    if let Ok(cache) = image_cache.lock() {
        if let Some(bytes) = cache.get(href).map(|x| x.clone()) {
            return Some(bytes);
        }
    }

    if let Ok(response) = ureq::get(href).call() {
        if response.status().is_success() {
            let mut buf = Vec::new();
            if response
                .into_body()
                .into_reader()
                .read_to_end(&mut buf)
                .is_ok()
            {
                let data = Arc::new(buf);

                if let Ok(mut cache) = image_cache.lock() {
                    cache.insert(href.to_owned(), data.clone());
                }

                return Some(data);
            }
        }
    }

    None
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

fn collect_bboxes(
    parent: &usvg::Group,
    bboxes: &mut Vec<usvg::Rect>,
    stroke_bboxes: &mut Vec<usvg::Rect>,
) {
    for node in parent.children() {
        if let usvg::Node::Group(ref group) = node {
            collect_bboxes(group, bboxes, stroke_bboxes);
        }

        let bbox = node.abs_bounding_box();
        bboxes.push(bbox);

        let stroke_bbox = node.abs_stroke_bounding_box();
        if bbox != stroke_bbox {
            stroke_bboxes.push(stroke_bbox);
        }
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
