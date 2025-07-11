use crate::layout::Renderable;

#[derive(Debug, Default)]
pub struct Row {
    pub spacing: Option<f32>,
    pub padding: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub reverse: Option<bool>,
}

impl Renderable for Row {
    fn to_svg(&self) -> &str {
        todo!()
    }
}

impl Row {
    pub fn new() -> Self {
        Self::default()
    }
}
