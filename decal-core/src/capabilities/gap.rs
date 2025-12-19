use super::Drawable;

pub trait Gap: Drawable {
    fn gap<T>(mut self, value: T) -> Self
    where
        T: crate::attributes::IntoGap,
    {
        self.layout_mut().gap = value.into_gap().unwrap_or_default().into();
        self
    }
}
