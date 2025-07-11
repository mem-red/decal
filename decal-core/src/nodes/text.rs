use crate::layout::Renderable;

#[derive(Debug, Default)]
pub struct Text {
    pub content: String,
}

impl Renderable for Text {
    fn to_svg(&self) -> &str {
        todo!()
    }
}
