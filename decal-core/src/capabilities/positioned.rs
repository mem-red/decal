use super::Drawable;
use crate::primitives::Position;

pub trait Positioned: Drawable {
    fn position(mut self, value: Position) -> Self {
        self.layout_mut().position = value.into();
        self
    }
}
