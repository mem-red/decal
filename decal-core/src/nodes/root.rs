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
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            background: None,
            foreground: None,
        }
    }

    pub fn set_background(&mut self, value: Option<Fill>) -> &mut Self {
        self.background = value;
        self
    }
}
