use crate::layout::{Node, NodeKind};
use taffy::prelude::*;

#[derive(Debug)]
pub struct Grid {
    style: Style,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            style: Style {
                display: Display::Grid,
                ..Default::default()
            },
        }
    }

    pub fn build(&self) -> Node {
        Node::new(NodeKind::Grid, self.style.to_owned())
    }
}
