use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_methods;
use crate::paint::Appearance;
use crate::prelude::Typography;
use taffy::prelude::*;

// TODO: impl grid layout methods

#[derive(Debug, Default)]
pub struct Grid {
    layout: Style,
    visual: Appearance,
    typography: Typography,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            layout: Style {
                display: Display::Grid,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn hidden(&mut self, value: bool) -> &mut Self {
        self.layout.display = if value { Display::None } else { Display::Grid };
        self
    }

    pub fn build(&self) -> Node {
        Node::new(
            NodeKind::Grid,
            self.layout.to_owned(),
            self.visual.to_owned(),
            Some(self.typography.to_owned()),
        )
    }
}

impl_node_methods!(
    Grid,
    [
        aspect_ratio,
        background,
        border,
        border_color,
        container_align,
        corner_radius,
        dimensions,
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
