use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_methods;
use crate::paint::Appearance;
use crate::prelude::Typography;
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

    pub fn build(&self) -> Node {
        Node::new(
            NodeKind::Root(self.meta.to_owned()),
            self.layout.to_owned(),
            self.visual.to_owned(),
            Some(self.typography.to_owned()),
        )
    }
}

impl_node_methods!(Root, [text]);
