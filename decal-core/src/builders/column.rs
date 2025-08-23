use crate::{
    attributes::Fill,
    layout::{Node, NodeKind},
    macros::{impl_margin_methods, impl_padding_methods},
};
use taffy::prelude::*;

#[derive(Debug)]
pub struct Column {
    style: Style,
    background: Fill,
}

impl Column {
    pub fn new() -> Self {
        Self {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background: Fill::default(),
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

    pub fn background(&mut self, value: Fill) -> &mut Self {
        self.background = value;
        self
    }

    pub fn build(&self) -> Node {
        Node::new(NodeKind::Column, self.style.to_owned())
    }
}

impl_padding_methods!(Column);
impl_margin_methods!(Column);
