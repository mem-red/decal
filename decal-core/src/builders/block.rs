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
    primitives::{
        Display,
        FlexDirection,
    },
};
use taffy::Style;

/// Block node.
#[derive(Debug, Default)]
pub struct Block {
    layout: Style,
    visual: Appearance,
    typography: Typography,
    resources: Vec<Resource>,
    /// Last non-none display value preserved for visibility toggling.
    last_display: taffy::Display,
}

impl_node_builder! {
    Block,
    build(this) {
        Node::new(
            NodeKind::Block,
            this.layout,
            this.visual,
            Some(this.typography),
            this.resources
        )
    }
}

impl Block {
    /// Creates a new block node.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn new() -> Self {
        Self {
            layout: Style {
                display: taffy::Display::Block,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Sets the display mode of the block.
    ///
    /// # Arguments
    /// - `display`: The [`Display`] property for the block.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn display(mut self, display: Display) -> Self {
        self.layout.display = display.into();
        self
    }

    /// Sets the flex direction used when the block is set to [`Display::Flex`].
    ///
    /// # Arguments
    /// - `direction`: The [`FlexDirection`] to apply.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn flex_direction(mut self, direction: FlexDirection) -> Self {
        self.layout.flex_direction = direction.into();
        self
    }

    /// Sets display property of the block to [`Display::Flex`] with direction
    /// set to [`FlexDirection::Column`].
    ///
    /// # Returns
    /// - [`Self`]
    pub fn flex_col(self) -> Self {
        self.display(Display::Flex)
            .flex_direction(FlexDirection::Column)
    }

    /// Sets display property of the block to [`Display::Flex`] with direction
    /// set to [`FlexDirection::Row`].
    ///
    /// # Returns
    /// - [`Self`]
    pub fn flex_row(self) -> Self {
        self.display(Display::Flex)
            .flex_direction(FlexDirection::Row)
    }
}

impl Hideable for Block {
    fn hidden(mut self, value: bool) -> Self {
        self.layout.display = if value {
            self.last_display = self.layout.display;
            taffy::Display::None
        } else {
            self.last_display
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
impl FilterEffects for Block {}
impl Blendable for Block {}
