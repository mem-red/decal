use crate::layout::Renderable;

#[derive(Debug, Default, Clone, Copy)]
pub struct Snippet;

impl Renderable for Snippet {
    fn to_svg(&self) -> &str {
        "" // TODO:
    }
}

impl Snippet {
    pub fn new() -> Self {
        Self::default()
    }
}
