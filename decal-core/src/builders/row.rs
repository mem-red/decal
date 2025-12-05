use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_methods;
use crate::paint::Appearance;
use crate::prelude::Typography;
use taffy::prelude::*;

#[derive(Debug, Default)]
pub struct Row {
    layout: Style,
    visual: Appearance,
    typography: Typography,
}

impl Row {
    pub fn new() -> Self {
        Self {
            layout: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn hidden(&mut self, value: bool) -> &mut Self {
        self.layout.display = if value { Display::None } else { Display::Flex };
        self
    }

    pub fn reverse(&mut self, reverse: bool) -> &mut Self {
        self.layout.flex_direction = if reverse {
            FlexDirection::RowReverse
        } else {
            FlexDirection::Row
        };
        self
    }

    pub fn build(&self) -> Node {
        Node::new(
            NodeKind::Row,
            self.layout.to_owned(),
            self.visual.to_owned(),
            Some(self.typography.to_owned()),
        )
    }
}

impl_node_methods!(
    Row,
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
        overflow,
        padding,
        position,
        self_align,
        text
    ]
);
