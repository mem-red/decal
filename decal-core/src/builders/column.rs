use crate::capabilities::*;
use crate::layout::Typography;
use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_builder;
use crate::paint::Appearance;
use crate::prelude::Resource;
use taffy::prelude::*;

#[derive(Debug, Default)]
pub struct Column {
    layout: Style,
    visual: Appearance,
    typography: Typography,
    resources: Vec<Resource>,
}

impl_node_builder! {
    Column,
    build(this) {
        Node::new(
            NodeKind::Column,
            this.layout,
            this.visual,
            Some(this.typography),
            this.resources
        )
    }
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
}

impl Hideable for Column {
    fn hidden(mut self, value: bool) -> Self {
        self.layout.display = if value { Display::None } else { Display::Flex };
        self
    }
}

impl FlexContainer for Column {
    fn reversed(mut self, reverse: bool) -> Self {
        self.layout.flex_direction = if reverse {
            FlexDirection::ColumnReverse
        } else {
            FlexDirection::Column
        };

        self
    }
}

impl AspectRatio for Column {}
impl Background for Column {}
impl Border for Column {}
impl RoundedCorners for Column {}
impl Dimensions for Column {}
impl Gap for Column {}
impl ContainerAlignment for Column {}
impl Margin for Column {}
impl Padding for Column {}
impl Clippable for Column {}
impl Opacity for Column {}
impl Positioned for Column {}
impl Transformation for Column {}
impl Textual for Column {}
impl SelfAlignment for Column {}
impl Visibility for Column {}
