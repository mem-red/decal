use crate::capabilities::*;
use crate::layout::{ImageMeta, ImageSource, Node, NodeKind, Typography};
use crate::macros::impl_node_builder;
use crate::paint::Appearance;
use taffy::prelude::*;

#[derive(Debug, Default)]
pub struct Image {
    meta: ImageMeta,
    layout: Style,
    visual: Appearance,
    // unused, only for satisfying the Sealed trait
    typography: Typography,
}

impl_node_builder! {
    Image,
    build(this) {
        Node::new(
            NodeKind::Image(this.meta.to_owned()),
            this.layout.to_owned(),
            this.visual.to_owned(),
            None
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
}

impl Hideable for Image {
    fn hidden(&mut self, value: bool) -> &mut Self {
        self.layout.display = if value { Display::None } else { Display::Block };
        self
    }
}

impl AspectRatio for Image {}
impl Background for Image {}
impl Border for Image {}
impl RoundedCorners for Image {}
impl Dimensions for Image {}
impl Margin for Image {}
impl Opacity for Image {}
impl Positioned for Image {}
impl Transformation for Image {}
impl SelfAlignment for Image {}
impl Visibility for Image {}
