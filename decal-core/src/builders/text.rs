use crate::layout::{Node, NodeKind};
use taffy::prelude::*;

#[derive(Debug)]
pub struct Text {
    content: String,
    style: Style,
}

impl Text {
    pub fn new<S>(content: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            content: content.into(),
            style: Style::default(),
        }
    }

    pub fn build(&self) -> Node {
        Node::new(
            NodeKind::Text {
                content: self.content.to_owned(),
            },
            self.style.to_owned(),
        )
    }
}
