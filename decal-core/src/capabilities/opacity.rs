use super::Drawable;

pub trait Opacity: Drawable {
    fn opacity<T>(mut self, value: T) -> Self
    where
        T: Into<f32>,
    {
        self.visual_mut().opacity = value.into().clamp(0.0, 1.0);
        self
    }
}
