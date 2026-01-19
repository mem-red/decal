use crate::capabilities::Textual;
use crate::layout::Typography;
use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_builder;
use crate::paint::Appearance;
use crate::paint::Resource;
use taffy::prelude::*;

#[derive(Debug, Default)]
pub struct Root {
    layout: Style,
    visual: Appearance,
    typography: Typography,
    resources: Vec<Resource>,
}

impl_node_builder!(
    Root,
    build(this) {
        Node::new(
            NodeKind::Root,
            this.layout,
            this.visual,
            Some(this.typography),
            this.resources
        )
    }
);

impl Root {
    pub fn new<T>(width: T, height: T) -> Self
    where
        T: Into<Option<f32>>,
    {
        Self {
            layout: Style {
                size: Size {
                    width: width.into().map(|x| length(x)).unwrap_or(Dimension::auto()),
                    height: height
                        .into()
                        .map(|x| length(x))
                        .unwrap_or(Dimension::auto()),
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Textual for Root {}
