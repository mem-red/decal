use crate::layout::{Node, NodeKind};
use taffy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Debug)]
pub struct Flex {
    style: Style,
}

impl Into<taffy::FlexDirection> for FlexDirection {
    fn into(self) -> taffy::FlexDirection {
        match self {
            Self::Row => taffy::FlexDirection::Row,
            Self::RowReverse => taffy::FlexDirection::RowReverse,
            Self::Column => taffy::FlexDirection::Column,
            Self::ColumnReverse => taffy::FlexDirection::ColumnReverse,
        }
    }
}

impl Flex {
    pub fn new(direction: FlexDirection) -> Self {
        Self {
            style: Style {
                display: Display::Flex,
                flex_direction: direction.into(),
                ..Default::default()
            },
        }
    }

    pub fn build(&self) -> Node {
        Node::new(NodeKind::Flex, self.style.to_owned())
    }
}
