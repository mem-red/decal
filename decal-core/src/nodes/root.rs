use crate::{attributes::Fill, layout::Renderable};

#[derive(Debug, Clone, Copy)]
pub struct Root {
    pub width: f32,
    pub height: f32,
    //
    pub background: Option<Fill>,
    pub foreground: Option<Fill>,
}

impl Renderable for Root {
    fn to_svg(&self) -> &str {
        todo!()
    }
}

impl Root {
    pub fn new<W, H>(width: W, height: H) -> Self
    where
        W: Into<f32>,
        H: Into<f32>,
    {
        Self {
            width: width.into(),
            height: height.into(),
            background: None,
            foreground: None,
        }
    }

    pub fn set_background(&mut self, value: Option<Fill>) -> &mut Self {
        self.background = value;
        self
    }
}