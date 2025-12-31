use crate::capabilities::*;
use crate::layout::Typography;
use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_builder;
use crate::paint::Appearance;
use crate::paint::Resource;
use taffy::prelude::*;

#[derive(Debug, Default)]
pub struct Row {
    layout: Style,
    visual: Appearance,
    typography: Typography,
    resources: Vec<Resource>,
}

impl_node_builder!(
    Row,
    build(this) {
        Node::new(
            NodeKind::Row,
            this.layout,
            this.visual,
            Some(this.typography),
            this.resources
        )
    }
);

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
}

impl Hideable for Row {
    fn hidden(mut self, value: bool) -> Self {
        self.layout.display = if value { Display::None } else { Display::Flex };
        self
    }
}

impl FlexContainer for Row {
    fn reversed(mut self, reverse: bool) -> Self {
        self.layout.flex_direction = if reverse {
            FlexDirection::RowReverse
        } else {
            FlexDirection::Row
        };

        self
    }
}

impl AspectRatio for Row {}
impl Background for Row {}
impl Border for Row {}
impl RoundedCorners for Row {}
impl Dimensions for Row {}
impl Gap for Row {}
impl ContainerAlignment for Row {}
impl Margin for Row {}
impl Padding for Row {}
impl Clippable for Row {}
impl Opacity for Row {}
impl Positioned for Row {}
impl Transformation for Row {}
impl Textual for Row {}
impl SelfAlignment for Row {}
impl Visibility for Row {}
impl FilterEffects for Row {}
