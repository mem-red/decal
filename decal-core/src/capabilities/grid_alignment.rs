use super::Drawable;
use crate::primitives::JustifyItems;

pub trait GridAlignment: Drawable {
    fn justify_items<T>(mut self, value: T) -> Self
    where
        T: Into<Option<JustifyItems>>,
    {
        self.layout_mut().justify_items = value.into().map(Into::into);
        self
    }
}
