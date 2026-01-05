use super::Drawable;
use crate::primitives::BlendMode;

pub trait Blendable: Drawable {
    fn blend_mode(mut self, value: BlendMode) -> Self {
        self.visual_mut().blend_mode = value;
        self
    }
}
