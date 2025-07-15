use crate::layout::Renderable;

#[derive(Debug, Default, Clone)]
pub struct Text {
    pub content: Option<String>,
}

impl Renderable for Text {
    fn to_svg(&self) -> &str {
        todo!()
    }
}

impl Text {
    pub fn new<S>(content: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            content: Some(content.into()),
        }
    }
}
