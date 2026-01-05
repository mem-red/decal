use crate::capabilities::*;
use crate::layout::Typography;
use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_builder;
use crate::paint::Appearance;
use crate::paint::Resource;
use taffy::prelude::*;

// TODO: impl grid layout

#[derive(Debug, Default)]
pub struct Grid {
    layout: Style,
    visual: Appearance,
    typography: Typography,
    resources: Vec<Resource>,
}

impl_node_builder! {
    Grid,
    build(this) {
        Node::new(
            NodeKind::Grid,
            this.layout,
            this.visual,
            Some(this.typography),
            this.resources
        )
    }
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
}

impl Hideable for Grid {
    fn hidden(mut self, value: bool) -> Self {
        self.layout.display = if value { Display::None } else { Display::Grid };
        self
    }
}

impl AspectRatio for Grid {}
impl Background for Grid {}
impl Border for Grid {}
impl RoundedCorners for Grid {}
impl Dimensions for Grid {}
impl Gap for Grid {}
impl ContainerAlignment for Grid {}
impl GridAlignment for Grid {}
impl Margin for Grid {}
impl Padding for Grid {}
impl Clippable for Grid {}
impl Opacity for Grid {}
impl Positioned for Grid {}
impl Transformation for Grid {}
impl Textual for Grid {}
impl SelfAlignment for Grid {}
impl Visibility for Grid {}
impl FilterEffects for Grid {}
impl Blendable for Grid {}
