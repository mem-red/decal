use super::Drawable;
use crate::primitives::{
    AlignSelf,
    IntoOptionalLength,
    JustifySelf,
};

pub trait SelfAlignment: Drawable {
    fn align_self<T>(mut self, value: T) -> Self
    where
        T: Into<Option<AlignSelf>>,
    {
        self.layout_mut().align_self = value.into().map(Into::into);
        self
    }

    fn justify_self<T>(mut self, value: T) -> Self
    where
        T: Into<Option<JustifySelf>>,
    {
        self.layout_mut().justify_self = value.into().map(Into::into);
        self
    }

    fn flex_basis<T>(mut self, value: T) -> Self
    where
        T: IntoOptionalLength,
    {
        self.layout_mut().flex_basis = value
            .into_optional_length()
            .map(Into::into)
            .unwrap_or(taffy::Dimension::auto());
        self
    }

    fn flex_grow<T>(mut self, value: T) -> Self
    where
        T: Into<Option<f32>>,
    {
        self.layout_mut().flex_grow = value.into().unwrap_or(0.0);
        self
    }

    fn flex_shrink<T>(mut self, value: T) -> Self
    where
        T: Into<Option<f32>>,
    {
        self.layout_mut().flex_shrink = value.into().unwrap_or(1.0);
        self
    }
}
