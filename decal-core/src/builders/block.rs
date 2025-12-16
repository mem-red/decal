use crate::layout::Typography;
use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_methods;
use crate::paint::Appearance;
use crate::primitives::{Display, FlexDirection};
use taffy::Style;

#[derive(Debug, Default)]
pub struct Block {
    layout: Style,
    visual: Appearance,
    typography: Typography,
    prev_display: taffy::Display,
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

    pub fn hidden(&mut self, value: bool) -> &mut Self {
        self.layout.display = if value {
            self.prev_display = self.layout.display;
            taffy::Display::None
        } else {
            self.prev_display
        };

        self
    }

    pub fn build(&self) -> Node {
        Node::new(
            NodeKind::Block,
            self.layout.to_owned(),
            self.visual.to_owned(),
            Some(self.typography.to_owned()),
        )
    }
}

impl_node_methods!(
    Block,
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
        grid_align,
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
