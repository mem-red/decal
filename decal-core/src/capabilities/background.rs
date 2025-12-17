use super::Drawable;

pub trait Background: Drawable {
    fn background<T>(&mut self, value: T) -> &mut Self
    where
        T: crate::attributes::IntoFill,
    {
        self.visual_mut().background = value.into_fill().unwrap_or(crate::primitives::Fill::None);
        self
    }

    fn bg<T>(&mut self, value: T) -> &mut Self
    where
        T: crate::attributes::IntoFill,
    {
        self.visual_mut().background = value.into_fill().unwrap_or(crate::primitives::Fill::None);
        self
    }
}
