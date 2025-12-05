use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_methods;
use crate::paint::Appearance;
use crate::prelude::Typography;
use taffy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Debug, Default)]
pub struct Flex {
    layout: Style,
    visual: Appearance,
    typography: Typography,
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
            layout: Style {
                display: Display::Flex,
                flex_direction: direction.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn hidden(&mut self, value: bool) -> &mut Self {
        self.layout.display = if value { Display::None } else { Display::Flex };
        self
    }

    pub fn build(&self) -> Node {
        Node::new(
            NodeKind::Flex,
            self.layout.to_owned(),
            self.visual.to_owned(),
            Some(self.typography.to_owned()),
        )
    }
}

impl_node_methods!(
    Flex,
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
