use crate::layout::Renderable;
use crate::macros::impl_node_setters;
use taffy::Dimension;

#[derive(Debug, Default, Clone, Copy)]
pub struct Column {
    pub spacing: Option<f32>,
    pub padding: Option<f32>,
    pub width: Option<Dimension>,
    pub height: Option<Dimension>,
    pub reverse: Option<bool>,
}

impl Renderable for Column {
    fn to_svg(&self) -> &str {
        todo!()
    }
}

impl Column {
    pub fn new() -> Self {
        Self::default()
    }
}

impl_node_setters!(Column, {
    /// The spacing between child nodes for the node.
    spacing: f32,
    ///
    padding: f32,
    ///
    width: Dimension,
    height: Dimension,
    reverse: bool
});
