use super::Drawable;
use crate::attributes::{IntoMargin, IntoMarginPair};
use crate::primitives::Length;

macro_rules! impl_side {
    ($method:ident, $field:ident) => {
        fn $method<T>(mut self, value: T) -> Self
        where
            T: Into<Option<Length>>,
        {
            self.layout_mut().margin.$field = value.into().unwrap_or_default().into();
            self
        }
    };
}

pub trait Margin: Drawable {
    fn margin<T>(mut self, value: T) -> Self
    where
        T: IntoMargin,
    {
        self.layout_mut().margin = value.into_margin().unwrap_or_default().into();
        self
    }

    fn margin_x<T>(mut self, value: T) -> Self
    where
        T: IntoMarginPair,
    {
        let (left, right) = value.into_margin_pair().unwrap_or_default();
        self.layout_mut().margin.left = left.into();
        self.layout_mut().margin.right = right.into();
        self
    }

    fn margin_y<T>(mut self, value: T) -> Self
    where
        T: IntoMarginPair,
    {
        let (top, bottom) = value.into_margin_pair().unwrap_or_default();
        self.layout_mut().margin.top = top.into();
        self.layout_mut().margin.bottom = bottom.into();
        self
    }

    impl_side!(margin_top, top);
    impl_side!(margin_right, right);
    impl_side!(margin_bottom, bottom);
    impl_side!(margin_left, left);
}
