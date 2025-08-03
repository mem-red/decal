use crate::layout::{Node, NodeKind};
use taffy::prelude::*;

#[derive(Debug)]
pub struct Row {
    style: Style,
}

impl Row {
    pub fn new() -> Self {
        Self {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
        }
    }

    pub fn reverse(&mut self, reverse: bool) -> &mut Self {
        self.style.flex_direction = if reverse {
            FlexDirection::RowReverse
        } else {
            FlexDirection::Row
        };
        self
    }

    pub fn build(&self) -> Node {
        Node::new(NodeKind::Row, self.style.to_owned())
    }
}
