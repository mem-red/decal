use super::Drawable;

pub trait Positioned: Drawable {
    fn position(mut self, value: crate::primitives::Position) -> Self {
        self.layout_mut().position = value.into();
        self
    }
}
