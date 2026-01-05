use crate::capabilities::*;
use crate::layout::Typography;
use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_builder;
use crate::paint::Appearance;
use crate::paint::Resource;
use crate::primitives::FlexDirection;
use taffy::{Display, Style};

#[derive(Debug, Default)]
pub struct Flex {
    layout: Style,
    visual: Appearance,
    typography: Typography,
    resources: Vec<Resource>,
}

impl_node_builder! {
    Flex,
    build(this) {
        Node::new(
            NodeKind::Flex,
            this.layout,
            this.visual,
            Some(this.typography),
            this.resources
        )
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
}

impl Hideable for Flex {
    fn hidden(mut self, value: bool) -> Self {
        self.layout.display = if value { Display::None } else { Display::Flex };
        self
    }
}

impl AspectRatio for Flex {}
impl Background for Flex {}
impl Border for Flex {}
impl RoundedCorners for Flex {}
impl Dimensions for Flex {}
impl Gap for Flex {}
impl FlexContainer for Flex {}
impl ContainerAlignment for Flex {}
impl Margin for Flex {}
impl Padding for Flex {}
impl Clippable for Flex {}
impl Opacity for Flex {}
impl Positioned for Flex {}
impl Transformation for Flex {}
impl Textual for Flex {}
impl SelfAlignment for Flex {}
impl Visibility for Flex {}
impl FilterEffects for Flex {}
impl Blendable for Flex {}
