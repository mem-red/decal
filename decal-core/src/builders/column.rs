use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_methods;
use crate::paint::Appearance;
use crate::prelude::Typography;
use taffy::prelude::*;

#[derive(Debug, Default)]
pub struct Column {
    layout: Style,
    visual: Appearance,
    typography: Typography,
}

impl Column {
    pub fn new() -> Self {
        Self {
            layout: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn reverse(&mut self, reverse: bool) -> &mut Self {
        self.layout.flex_direction = if reverse {
            FlexDirection::ColumnReverse
        } else {
            FlexDirection::Column
        };
        self
    }

    pub fn hidden(&mut self, value: bool) -> &mut Self {
        self.layout.display = if value { Display::None } else { Display::Flex };
        self
    }

    pub fn build(&self) -> Node {
        Node::new(
            NodeKind::Column,
            self.layout.to_owned(),
            self.visual.to_owned(),
            Some(self.typography.to_owned()),
        )
    }
}

impl_node_methods!(
    Column,
    [
        aspect_ratio,
        background,
        border,
        border_color,
        container_align,
        corner_radius,
        dimensions,
        flex_wrap,
        gap,
        margin,
        opacity,
        overflow,
        padding,
        position,
        self_align,
        text,
        transform,
        visibility
    ]
);
