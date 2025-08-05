use crate::layout::{Node, NodeKind};
use taffy::prelude::*;

#[derive(Debug)]
pub struct Root {
    meta: RootMeta,
    style: Style,
}

#[derive(Debug, Clone)]
pub(crate) struct RootMeta {
    width: f32,
    height: f32,
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
            style: Style {
                size: Size {
                    width: length(width),
                    height: length(height),
                },
                ..Default::default()
            },
        }
    }

    pub fn build(&self) -> Node {
        Node::new(NodeKind::Root(self.meta.to_owned()), self.style.to_owned())
    }
}
