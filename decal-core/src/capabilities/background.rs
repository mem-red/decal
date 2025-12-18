use super::Drawable;

pub trait Background: Drawable {
    fn background<T>(&mut self, value: T) -> &mut Self
    where
        T: crate::attributes::IntoPaint,
    {
        self.visual_mut().background = value.into_paint().unwrap_or(crate::primitives::Paint::None);
        self
    }

    fn bg<T>(&mut self, value: T) -> &mut Self
    where
        T: crate::attributes::IntoPaint,
    {
        self.visual_mut().background = value.into_paint().unwrap_or(crate::primitives::Paint::None);
        self
    }
}
