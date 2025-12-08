use crate::layout::{ImageMeta, Node, NodeKind};
use crate::macros::impl_node_methods;
use crate::paint::Appearance;
use taffy::prelude::*;

#[derive(Debug, Default)]
pub struct Image {
    meta: ImageMeta,
    layout: Style,
    visual: Appearance,
}

impl Image {
    pub fn new<S>(source: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            meta: ImageMeta::new(source),
            layout: Style::default(),
            ..Default::default()
        }
    }

    pub fn hidden(&mut self, value: bool) -> &mut Self {
        self.layout.display = if value { Display::None } else { Display::Block };
        self
    }

    pub fn build(&self) -> Node {
        Node::new(
            NodeKind::Image(self.meta.to_owned()),
            self.layout.to_owned(),
            self.visual.to_owned(),
            None,
        )
    }
}

impl_node_methods!(
    Image,
    [
        aspect_ratio,
        background,
        border,
        border_color,
        corner_radius,
        dimensions,
        margin,
        position,
        self_align,
        transform
    ]
);
