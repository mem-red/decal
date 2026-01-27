use crate::{
    capabilities::*,
    layout::{
        ImageMeta,
        ImageSource,
        Node,
        NodeKind,
        Typography,
    },
    macros::impl_node_builder,
    paint::{
        Appearance,
        Resource,
    },
    primitives::CrossOrigin,
};
use taffy::prelude::*;

/// Image node.
#[derive(Debug, Default)]
pub struct Image {
    meta: ImageMeta,
    layout: Style,
    visual: Appearance,
    typography: Typography, // unused, only for satisfying the [`Sealed`] trait
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
    /// Creates a new image node with a fixed intrinsic size.
    ///
    /// # Arguments
    /// - `source`: The [`ImageSource`] value.
    /// - `width`: The intrinsic width of the image.
    /// - `height`: The intrinsic height of the image.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn new<S>(source: S, width: f32, height: f32) -> Self
    where
        S: Into<ImageSource>,
    {
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

    /// Sets the cross-origin policy used when loading the image.
    ///
    /// # Arguments
    /// - `cross_origin`: The [`CrossOrigin`] value.
    ///
    /// # Returns
    /// - [`Self`]
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
impl FilterEffects for Image {}
impl Blendable for Image {}
