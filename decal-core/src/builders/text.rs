use crate::layout::{Node, NodeKind};
use taffy::prelude::*;

#[derive(Debug)]
pub struct Text {
    meta: TextMeta,
    style: Style,
}

#[derive(Debug, Clone)]
pub(crate) struct TextMeta {
    pub(crate) content: String,
}

impl Text {
    pub fn new<S>(content: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            meta: TextMeta {
                content: content.into(),
            },
            style: Style::default(),
        }
    }

    pub fn build(&self) -> Node {
        Node::new(NodeKind::Text(self.meta.to_owned()), self.style.to_owned())
    }
}
