use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_methods;
use crate::paint::Appearance;
use crate::prelude::Typography;
use taffy::prelude::*;

#[derive(Debug, Default)]
pub struct Block {
    layout: Style,
    visual: Appearance,
    typography: Typography,
    prev_display: Display,
}

impl Block {
    pub fn new() -> Self {
        Self {
            layout: Style {
                display: Display::Block,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn display(&mut self, display: Display) -> &mut Self {
        self.layout.display = display;
        self
    }

    pub fn hidden(&mut self, value: bool) -> &mut Self {
        self.layout.display = if value {
            self.prev_display = self.layout.display;
            Display::None
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
        overflow,
        padding,
        position,
        self_align,
        text
    ]
);
