use crate::capabilities::*;
use crate::layout::Typography;
use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_builder;
use crate::paint::Appearance;
use crate::primitives::{Display, FlexDirection};
use taffy::Style;

#[derive(Debug, Default)]
pub struct Block {
    layout: Style,
    visual: Appearance,
    typography: Typography,
    //
    _prev_display: taffy::Display,
}

impl_node_builder! {
    Block,
    build(this) {
        Node::new(
            NodeKind::Block,
            this.layout.to_owned(),
            this.visual.to_owned(),
            Some(this.typography.to_owned()),
        )
    }
}

impl Block {
    pub fn new() -> Self {
        Self {
            layout: Style {
                display: taffy::Display::Block,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn display(&mut self, display: Display) -> &mut Self {
        self.layout.display = display.into();
        self
    }

    pub fn flex_direction(&mut self, direction: FlexDirection) -> &mut Self {
        self.layout.flex_direction = direction.into();
        self
    }

    //

    pub fn flex_col(&mut self) -> &mut Self {
        self.display(Display::Flex);
        self.flex_direction(FlexDirection::Column);
        self
    }

    pub fn flex_row(&mut self) -> &mut Self {
        self.display(Display::Flex);
        self.flex_direction(FlexDirection::Row);
        self
    }
}

impl Hideable for Block {
    fn hidden(&mut self, value: bool) -> &mut Self {
        self.layout.display = if value {
            self._prev_display = self.layout.display;
            taffy::Display::None
        } else {
            self._prev_display
        };

        self
    }
}

impl AspectRatio for Block {}
impl Background for Block {}
impl Border for Block {}
impl RoundedCorners for Block {}
impl Dimensions for Block {}
impl Gap for Block {}
impl ContainerAlignment for Block {}
impl FlexContainer for Block {}
impl GridAlignment for Block {}
impl SelfAlignment for Block {}
impl Margin for Block {}
impl Padding for Block {}
impl Clippable for Block {}
impl Opacity for Block {}
impl Visibility for Block {}
impl Positioned for Block {}
impl Transformation for Block {}
impl Textual for Block {}
