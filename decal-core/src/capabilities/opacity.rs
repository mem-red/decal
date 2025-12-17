use super::Drawable;

pub trait Opacity: Drawable {
    fn opacity(&mut self, value: f32) -> &mut Self {
        debug_assert!(value >= 0.0 && value <= 1.0);
        self.visual_mut().opacity = value.clamp(0.0, 1.0);
        self
    }
}
