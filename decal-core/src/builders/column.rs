use crate::layout::{Node, NodeKind};
use taffy::prelude::*;

#[derive(Debug)]
pub struct Column {
    style: Style,
}

impl Column {
    pub fn new() -> Self {
        Self {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
        }
    }

    pub fn reverse(&mut self, reverse: bool) -> &mut Self {
        self.style.flex_direction = if reverse {
            FlexDirection::ColumnReverse
        } else {
            FlexDirection::Column
        };
        self
    }

    pub fn build(&self) -> Node {
        Node::new(NodeKind::Column, self.style.to_owned())
    }
}
