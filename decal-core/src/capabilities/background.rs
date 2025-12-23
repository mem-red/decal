use super::Drawable;
use crate::attributes::IntoPaint;

pub trait Background: Drawable {
    fn background<T>(mut self, value: T) -> Self
    where
        T: IntoPaint,
    {
        let background = value.into_paint().unwrap_or(crate::primitives::Paint::None);
        self.visual_mut().background = background.clone();
        self.add_resource(background);

        self
    }

    fn background_opacity<T>(mut self, value: T) -> Self
    where
        T: Into<f32>,
    {
        self.visual_mut().background_opacity = value.into().clamp(0.0, 1.0);
        self
    }

    //

    fn bg<T>(self, value: T) -> Self
    where
        T: IntoPaint,
    {
        self.background(value)
    }

    fn bg_opacity<T>(self, value: T) -> Self
    where
        T: Into<f32>,
    {
        self.background_opacity(value)
    }
}
