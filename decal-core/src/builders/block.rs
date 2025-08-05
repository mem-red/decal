use crate::layout::{Node, NodeKind};
use taffy::prelude::*;

#[derive(Debug)]
pub struct Block {
    style: Style,
}

impl Block {
    pub fn new() -> Self {
        Self {
            style: Style {
                display: Display::Block,
                ..Default::default()
            },
        }
    }

    pub fn build(&self) -> Node {
        Node::new(NodeKind::Block, self.style.to_owned())
    }
}
