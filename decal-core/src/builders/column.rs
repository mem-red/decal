use crate::{
    layout::{Node, NodeKind},
    prelude::{IntoPadding, Padding},
};
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

    pub fn padding<T>(&mut self, value: T) -> &mut Self
    where
        T: IntoPadding,
    {
        self.style.padding = value
            .into_padding()
            .map_or(taffy::Rect::zero(), |inner| inner.to_style());
        self
    }

    pub fn build(&self) -> Node {
        Node::new(NodeKind::Column, self.style.to_owned())
    }
}
