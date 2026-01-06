use super::Drawable;

pub trait Opacity: Drawable {
    fn opacity(mut self, value: f32) -> Self {
        self.visual_mut().opacity = value.clamp(0.0, 1.0);
        self
    }
}
