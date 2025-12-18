use super::Drawable;

pub trait Background: Drawable {
    fn background<T>(&mut self, value: T) -> &mut Self
    where
        T: crate::attributes::IntoPaint,
    {
        self.visual_mut().background = value.into_paint().unwrap_or(crate::primitives::Paint::None);
        self
    }

    fn background_opacity<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<f32>,
    {
        self.visual_mut().background_opacity = value.into().clamp(0.0, 1.0);
        self
    }

    //

    fn bg<T>(&mut self, value: T) -> &mut Self
    where
        T: crate::attributes::IntoPaint,
    {
        self.visual_mut().background = value.into_paint().unwrap_or(crate::primitives::Paint::None);
        self
    }

    fn bg_opacity<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<f32>,
    {
        self.visual_mut().background_opacity = value.into().clamp(0.0, 1.0);
        self
    }
}
