use super::Drawable;
use crate::attributes::IntoGap;

pub trait Gap: Drawable {
    fn gap<T>(mut self, value: T) -> Self
    where
        T: IntoGap,
    {
        self.layout_mut().gap = value.into_gap().unwrap_or_default().into();
        self
    }
}
