use crate::builders::{Column, Fragment, Row, Text};
use taffy::{
    Cache, CacheTree, compute_cached_layout, compute_flexbox_layout, compute_leaf_layout,
    compute_root_layout, prelude::*,
};

pub(crate) trait Renderable {
    fn to_svg(&self) -> &str;
}

// fn main() -> Result<(), taffy::TaffyError> {
//     let mut taffy: TaffyTree<()> = TaffyTree::new();

//     let child = taffy.new_leaf(Style {
//         size: Size {
//             width: Dimension::from_percent(0.5),
//             height: Dimension::AUTO,
//         },
//         ..Default::default()
//     })?;

//     let node = taffy.new_with_children(
//         Style {
//             size: Size {
//                 width: Dimension::from_length(100.0),
//                 height: Dimension::from_length(100.0),
//             },
//             // padding,
//             justify_content: Some(JustifyContent::Center),
//             ..Default::default()
//         },
//         &[child],
//     )?;

//     println!("Compute layout with 100x100 viewport:");
//     taffy.compute_layout(
//         node,
//         Size {
//             height: AvailableSpace::Definite(100.0),
//             width: AvailableSpace::Definite(100.0),
//         },
//     )?;
//     println!("node: {:#?}", taffy.layout(node)?);
//     println!("child: {:#?}", taffy.layout(child)?);

//     println!("Compute layout with undefined (infinite) viewport:");
//     taffy.compute_layout(node, Size::MAX_CONTENT)?;
//     println!("node: {:#?}", taffy.layout(node)?);
//     println!("child: {:#?}", taffy.layout(child)?);

//     Ok(())
// }

// #[derive(Debug, Clone)]
// pub struct Decal {
//     width: f32,
//     height: f32,
//     root: Node,
// }

// impl Decal {
//     pub fn new<W, H>(width: W, height: H, root: Node) -> Self
//     where
//         W: Into<f64>,
//         H: Into<f64>,
//     {
//         Self {
//             width: width.into() as f32,
//             height: height.into() as f32,
//             root,
//         }
//     }

//     pub fn root(&mut self) -> &mut Node {
//         &mut self.root
//     }

//     fn compute_layout(&mut self, available_space: Size<AvailableSpace>) {
//         self.root.compute_layout(available_space);
//     }
// }

// #[derive(Debug, Clone)]
// pub struct DecalFragment {
//     root: Node,
// }

// impl DecalFragment {
//     pub fn new(root: Node) -> Self {
//         Self { root }
//     }

//     pub fn root(&mut self) -> &mut Node {
//         &mut self.root
//     }
// }

#[derive(Debug, Clone)]
pub(crate) enum NodeKind {
    Root,
    Fragment,
    Column,
    Row,
    Text { content: String },
}

#[derive(Debug, Clone)]
pub struct Node {
    kind: NodeKind,
    style: Style,
    children: Vec<Node>,
    // Computed
    cache: Cache,
    layout: Layout,
}

impl Node {
    pub(crate) fn new(kind: NodeKind, style: Style) -> Self {
        Self {
            kind,
            style,
            children: Vec::new(),
            cache: Cache::new(),
            layout: Layout::with_order(0),
        }
    }

    pub fn append_child(&mut self, child: Node) -> &mut Node {
        let child_idx = self.children.len();
        self.children.push(child);
        &mut self.children[child_idx]
    }

    //

    pub fn compute_layout(&mut self, available_space: Size<AvailableSpace>) {
        // TODO:
        if !matches!(self.kind, NodeKind::Root) {
            panic!("can only be called on root node");
        }

        compute_root_layout(self, NodeId::from(usize::MAX), available_space);
    }

    /// The methods on LayoutPartialTree need to be able to access:
    ///
    ///  - The node being laid out
    ///  - Direct children of the node being laid out
    ///
    /// Each must have an ID. For children we simply use it's index. For the node itself
    /// we use usize::MAX on the assumption that there will never be that many children.
    fn node_from_id(&self, node_id: NodeId) -> &Node {
        let idx = usize::from(node_id);
        if idx == usize::MAX {
            self
        } else {
            &self.children[idx]
        }
    }

    fn node_from_id_mut(&mut self, node_id: NodeId) -> &mut Node {
        let idx = usize::from(node_id);
        if idx == usize::MAX {
            self
        } else {
            &mut self.children[idx]
        }
    }
}

pub struct ChildIter(std::ops::Range<usize>);

impl Iterator for ChildIter {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(NodeId::from)
    }
}

impl taffy::TraversePartialTree for Node {
    type ChildIter<'a> = ChildIter;

    fn child_ids(&self, _node_id: NodeId) -> Self::ChildIter<'_> {
        ChildIter(0..self.children.len())
    }

    fn child_count(&self, _node_id: NodeId) -> usize {
        self.children.len()
    }

    fn get_child_id(&self, _node_id: NodeId, index: usize) -> NodeId {
        NodeId::from(index)
    }
}

impl taffy::LayoutPartialTree for Node {
    type CoreContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;

    fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_> {
        &self.node_from_id(node_id).style
    }

    fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout) {
        self.node_from_id_mut(node_id).layout = *layout
    }

    fn resolve_calc_value(&self, _val: *const (), _basis: f32) -> f32 {
        0.0
    }

    fn compute_child_layout(
        &mut self,
        node_id: NodeId,
        inputs: taffy::tree::LayoutInput,
    ) -> taffy::tree::LayoutOutput {
        compute_cached_layout(self, node_id, inputs, |parent, node_id, inputs| {
            let node = parent.node_from_id_mut(node_id);
            // let font_metrics = FontMetrics {
            //     char_width: 10.0,
            //     char_height: 10.0,
            // };

            match node.kind {
                NodeKind::Column | NodeKind::Row => compute_flexbox_layout(node, node_id, inputs),
                // NodeKind::Grid => compute_grid_layout(node, node_id, inputs),
                NodeKind::Text { .. } => compute_leaf_layout(
                    inputs,
                    &node.style,
                    |_val, _basis| 0.0,
                    |known_dimensions, available_space| {
                        todo!()
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

impl CacheTree for Node {
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

impl taffy::LayoutFlexboxContainer for Node {
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

impl taffy::LayoutGridContainer for Node {
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

// impl Renderable for Node {
//     fn to_svg(&self) -> &str {
//         match self.kind {
//             NodeKind::Root(root) => root.to_svg(),
//            NodeKind:: Fragment(frag) => todo!(),
//         NodeKind::       Snippet(_) => todo!() ,
//           NodeKind::     Column(Column),
//            NodeKind::    Row(Row),
//             NodeKind::   Text(Text),
//         }
//     }
// }
