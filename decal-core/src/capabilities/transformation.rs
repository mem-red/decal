use super::Drawable;
use crate::primitives::Transform;

pub trait Transformation: Drawable {
    fn transform<T>(mut self, value: T) -> Self
    where
        T: Into<Option<Transform>>,
    {
        self.visual_mut().transform = value.into().unwrap_or_default();
        self
    }
}
