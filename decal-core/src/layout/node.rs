use crate::builders::{RootMeta, TextMeta};
use taffy::{
    Cache, CacheTree, compute_block_layout, compute_cached_layout, compute_flexbox_layout,
    compute_leaf_layout, compute_root_layout, prelude::*, print_tree, round_layout,
};

const ROOT_ID: usize = 0;

pub(crate) trait Renderable {
    fn to_svg(&self) -> &str;
}

pub struct Decal {
    nodes: Vec<Node>,
}

impl Decal {
    pub fn new(node: Node) -> Self {
        Self { nodes: vec![node] }
    }

    pub fn root_id(&mut self) -> usize {
        ROOT_ID
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn append_child(&mut self, parent_id: usize, child: Node) -> usize {
        self.nodes.push(child);
        let child_id = self.nodes.len() - 1;
        self.nodes[parent_id].children.push(child_id);
        child_id
    }

    pub fn append_fragment(&mut self, parent_id: usize, fragment: Decal) {
        if fragment.nodes.is_empty() {
            return;
        }

        let root_id = self.nodes.len(); // Fragment root node
        self.nodes[parent_id].children.push(root_id);
        self.nodes.reserve(fragment.nodes.len()); // Pre-allocation

        for mut node in fragment.nodes {
            // Update child indices after adding them to main arena.
            for child_id in node.children.iter_mut() {
                *child_id += root_id;
            }

            self.nodes.push(node);
        }
    }

    pub fn compute_layout(&mut self, available_space: Size<AvailableSpace>, use_rounding: bool) {
        // let root_node = &self.nodes[ROOT_ID];
        compute_root_layout(self, NodeId::from(ROOT_ID), available_space);

        // root_node;

        if use_rounding {
            round_layout(self, NodeId::from(ROOT_ID))
        }
    }

    pub fn print_tree(&mut self) {
        print_tree(self, NodeId::from(ROOT_ID));
    }

    //

    #[inline(always)]
    fn node_from_id(&self, node_id: NodeId) -> &Node {
        &self.nodes[usize::from(node_id)]
    }

    #[inline(always)]
    fn node_from_id_mut(&mut self, node_id: NodeId) -> &mut Node {
        &mut self.nodes[usize::from(node_id)]
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NodeKind {
    Root(RootMeta),
    Flex,
    Column,
    Row,
    Text(TextMeta),
}

#[derive(Debug, Clone)]
pub struct Node {
    kind: NodeKind,
    style: Style,
    children: Vec<usize>,
    // Computed
    cache: Cache,
    unrounded_layout: Layout,
    final_layout: Layout,
}

impl Node {
    pub(crate) fn new(kind: NodeKind, style: Style) -> Self {
        Self {
            kind,
            style,
            children: Vec::new(),
            cache: Cache::new(),
            unrounded_layout: Layout::with_order(0),
            final_layout: Layout::with_order(0),
        }
    }
}

pub struct ChildIter<'a>(std::slice::Iter<'a, usize>);

impl Iterator for ChildIter<'_> {
    type Item = NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied().map(NodeId::from)
    }
}

impl taffy::TraversePartialTree for Decal {
    type ChildIter<'a> = ChildIter<'a>;

    fn child_ids(&self, node_id: NodeId) -> Self::ChildIter<'_> {
        ChildIter(self.node_from_id(node_id).children.iter())
    }

    fn child_count(&self, node_id: NodeId) -> usize {
        self.node_from_id(node_id).children.len()
    }

    fn get_child_id(&self, node_id: NodeId, index: usize) -> NodeId {
        NodeId::from(self.node_from_id(node_id).children[index])
    }
}

impl taffy::TraverseTree for Decal {}

impl taffy::LayoutPartialTree for Decal {
    // type CustomIdent = String;

    type CoreContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;

    fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_> {
        &self.node_from_id(node_id).style
    }

    fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout) {
        self.node_from_id_mut(node_id).unrounded_layout = *layout;
    }

    fn resolve_calc_value(&self, _val: *const (), _basis: f32) -> f32 {
        0.0
    }

    fn compute_child_layout(
        &mut self,
        node_id: NodeId,
        inputs: taffy::tree::LayoutInput,
    ) -> taffy::tree::LayoutOutput {
        compute_cached_layout(self, node_id, inputs, |tree, node_id, inputs| {
            let node = &mut tree.nodes[usize::from(node_id)];
            // let font_metrics = FontMetrics {
            //     char_width: 10.0,
            //     char_height: 10.0,
            // };

            match node.kind {
                NodeKind::Root(_) => compute_block_layout(tree, node_id, inputs),
                NodeKind::Column | NodeKind::Row => compute_flexbox_layout(tree, node_id, inputs),
                // NodeKind::Grid => compute_grid_layout(node, node_id, inputs),
                NodeKind::Text(_) => compute_leaf_layout(
                    inputs,
                    &node.style,
                    |_val, _basis| 0.0,
                    |known_dimensions, available_space| {
                        // TODO:
                        Size::zero()
                        // text_measure_function(
                        //     known_dimensions,
                        //     available_space,
                        //     node.text_data.as_ref().unwrap(),
                        //     &font_metrics,
                        // )
                    },
                ),
                _ => unreachable!(),
                // NodeKind::Image => compute_leaf_layout(
                //     inputs,
                //     &node.style,
                //     |_val, _basis| 0.0,
                //     |known_dimensions, _available_space| {
                //         image_measure_function(known_dimensions, node.image_data.as_ref().unwrap())
                //     },
                // ),
            }
        })
    }
}

impl CacheTree for Decal {
    fn cache_get(
        &self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        run_mode: taffy::RunMode,
    ) -> Option<taffy::LayoutOutput> {
        self.node_from_id(node_id)
            .cache
            .get(known_dimensions, available_space, run_mode)
    }

    fn cache_store(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
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

    fn cache_clear(&mut self, node_id: NodeId) {
        self.node_from_id_mut(node_id).cache.clear();
    }
}

impl taffy::LayoutBlockContainer for Decal {
    type BlockContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;

    type BlockItemStyle<'a>
        = &'a Style
    where
        Self: 'a;

    fn get_block_container_style(&self, node_id: NodeId) -> Self::BlockContainerStyle<'_> {
        &self.node_from_id(node_id).style
    }

    fn get_block_child_style(&self, child_node_id: NodeId) -> Self::BlockItemStyle<'_> {
        &self.node_from_id(child_node_id).style
    }
}

impl taffy::LayoutFlexboxContainer for Decal {
    type FlexboxContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;

    type FlexboxItemStyle<'a>
        = &'a Style
    where
        Self: 'a;

    fn get_flexbox_container_style(&self, node_id: NodeId) -> Self::FlexboxContainerStyle<'_> {
        &self.node_from_id(node_id).style
    }

    fn get_flexbox_child_style(&self, child_node_id: NodeId) -> Self::FlexboxItemStyle<'_> {
        &self.node_from_id(child_node_id).style
    }
}

impl taffy::LayoutGridContainer for Decal {
    type GridContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;

    type GridItemStyle<'a>
        = &'a Style
    where
        Self: 'a;

    fn get_grid_container_style(&self, node_id: NodeId) -> Self::GridContainerStyle<'_> {
        &self.node_from_id(node_id).style
    }

    fn get_grid_child_style(&self, child_node_id: NodeId) -> Self::GridItemStyle<'_> {
        &self.node_from_id(child_node_id).style
    }
}

impl taffy::RoundTree for Decal {
    fn get_unrounded_layout(&self, node_id: NodeId) -> &Layout {
        &self.node_from_id(node_id).unrounded_layout
    }

    fn set_final_layout(&mut self, node_id: NodeId, layout: &Layout) {
        self.node_from_id_mut(node_id).final_layout = *layout;
    }
}

impl taffy::PrintTree for Decal {
    fn get_debug_label(&self, node_id: NodeId) -> &'static str {
        match self.node_from_id(node_id).kind {
            NodeKind::Root(_) => "ROOT",
            NodeKind::Flex => "FLEX",
            NodeKind::Column => "COLUMN",
            NodeKind::Row => "ROW",
            NodeKind::Text(_) => "TEXT",
        }
    }

    fn get_final_layout(&self, node_id: NodeId) -> &Layout {
        &self.node_from_id(node_id).final_layout
    }
}
