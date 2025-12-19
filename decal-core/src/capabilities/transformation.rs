use super::Drawable;

pub trait Transformation: Drawable {
    fn transform<T>(mut self, value: T) -> Self
    where
        T: Into<Option<crate::primitives::Transform>>,
    {
        self.visual_mut().transform = value.into().unwrap_or_default();
        self
    }
}
