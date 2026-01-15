use super::Drawable;
use crate::primitives::{AlignContent, AlignItems, JustifyContent};

pub trait ContainerAlignment: Drawable {
    fn align_items<T>(mut self, value: T) -> Self
    where
        T: Into<Option<AlignItems>>,
    {
        self.layout_mut().align_items = value.into().map(Into::into);
        self
    }

    fn align_content<T>(mut self, value: T) -> Self
    where
        T: Into<Option<AlignContent>>,
    {
        self.layout_mut().align_content = value.into().map(Into::into);
        self
    }

    fn justify_content<T>(mut self, value: T) -> Self
    where
        T: Into<Option<JustifyContent>>,
    {
        self.layout_mut().justify_content = value.into().map(Into::into);
        self
    }
}
