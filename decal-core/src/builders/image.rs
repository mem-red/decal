use crate::capabilities::*;
use crate::layout::{ImageMeta, ImageSource, Node, NodeKind, Typography};
use crate::macros::impl_node_builder;
use crate::paint::Appearance;
use crate::paint::Resource;
use crate::primitives::CrossOrigin;
use taffy::prelude::*;

#[derive(Debug, Default)]
pub struct Image {
    meta: ImageMeta,
    layout: Style,
    visual: Appearance,
    typography: Typography, // unused, only for satisfying the Sealed trait
    resources: Vec<Resource>,
}

impl_node_builder! {
    Image,
    build(this) {
        Node::new(
            NodeKind::Image(this.meta),
            this.layout,
            this.visual,
            None,
            this.resources
        )
    }
}

impl Image {
    pub fn new<S, T>(source: S, width: T, height: T) -> Self
    where
        S: Into<ImageSource>,
        T: Into<f32> + Copy,
    {
        let width = width.into();
        let height = height.into();

        Self {
            meta: ImageMeta::new(source, width, height),
            layout: Style {
                size: Size {
                    width: Dimension::length(width),
                    height: Dimension::length(height),
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn cross_origin<T>(mut self, cross_origin: T) -> Self
    where
        T: Into<Option<CrossOrigin>>,
    {
        self.meta.cross_origin = cross_origin.into();
        self
    }
}

impl Hideable for Image {
    fn hidden(mut self, value: bool) -> Self {
        self.layout.display = if value { Display::None } else { Display::Block };
        self
    }
}

impl AspectRatio for Image {}
impl Border for Image {}
impl RoundedCorners for Image {}
impl Dimensions for Image {}
impl Margin for Image {}
impl Opacity for Image {}
impl Positioned for Image {}
impl Transformation for Image {}
impl SelfAlignment for Image {}
impl Visibility for Image {}
