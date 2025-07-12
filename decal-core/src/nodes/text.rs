use crate::layout::Renderable;

#[derive(Debug, Default, Clone, Copy)]
pub struct Text {
    pub content: Option<u32>,
}

impl Renderable for Text {
    fn to_svg(&self) -> &str {
        todo!()
    }
}
