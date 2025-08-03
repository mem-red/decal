use crate::layout::{Node, NodeKind};
use taffy::prelude::*;

#[derive(Debug)]
pub struct Root {
    width: f32,
    height: f32,
    style: Style,
}

impl Root {
    pub fn new<W, H>(width: W, height: H) -> Self
    where
        W: Into<f64>,
        H: Into<f64>,
    {
        Self {
            width: width.into() as f32,
            height: height.into() as f32,
            style: Style::default(),
        }
    }

    pub fn build(&self) -> Node {
        Node::new(NodeKind::Column, self.style.to_owned())
    }
}
