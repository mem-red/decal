use crate::layout::{Node, NodeKind};
use taffy::prelude::*;

#[derive(Debug)]
pub struct Image {
    meta: ImageMeta,
    style: Style,
}

#[derive(Debug, Clone)]
pub(crate) struct ImageMeta {
    pub(crate) source: String,
}

impl Image {
    pub fn new<S>(source: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            meta: ImageMeta {
                source: source.into(),
            },
            style: Style::default(),
        }
    }

    pub fn build(&self) -> Node {
        Node::new(NodeKind::Image(self.meta.to_owned()), self.style.to_owned())
    }
}
