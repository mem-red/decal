use crate::capabilities::Textual;
use crate::layout::Typography;
use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_builder;
use crate::paint::Appearance;
use taffy::prelude::*;

#[derive(Debug, Default)]
pub struct Root {
    meta: RootMeta,
    layout: Style,
    visual: Appearance,
    typography: Typography,
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
            NodeKind::Root(this.meta.to_owned()),
            this.layout.to_owned(),
            this.visual.to_owned(),
            Some(this.typography.to_owned()),
        )
    }
);

impl Root {
    pub fn new<W, H>(width: W, height: H) -> Self
    where
        W: Into<f64>,
        H: Into<f64>,
    {
        let width = width.into() as f32;
        let height = height.into() as f32;

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
