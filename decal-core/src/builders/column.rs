use crate::{
    capabilities::*,
    layout::{
        Node,
        NodeKind,
        Typography,
    },
    macros::impl_node_builder,
    paint::{
        Appearance,
        Resource,
    },
};
use taffy::prelude::*;

/// Vertical flex container node.
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
    /// Creates a new [`Column`] node.
    ///
    /// # Returns
    /// - [`Self`]
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
impl FilterEffects for Column {}
impl Blendable for Column {}
