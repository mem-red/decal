use crate::capabilities::Textual;
use crate::layout::Typography;
use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_builder;
use crate::paint::Appearance;
use crate::paint::Resource;
use taffy::prelude::*;

#[derive(Debug, Default)]
pub struct Root {
    meta: RootMeta,
    layout: Style,
    visual: Appearance,
    typography: Typography,
    resources: Vec<Resource>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct RootMeta {
    pub(crate) width: f32,
    pub(crate) height: f32,
}

impl_node_builder!(
    Root,
    build(this) {
        Node::new(
            NodeKind::Root(this.meta),
            this.layout,
            this.visual,
            Some(this.typography),
            this.resources
        )
    }
);

impl Root {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            meta: RootMeta { width, height },
            layout: Style {
                size: Size {
                    width: length(width),
                    height: length(height),
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Textual for Root {}
