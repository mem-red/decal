use super::Drawable;
use crate::attributes::{IntoPadding, IntoPaddingPair, IntoPaddingValue};

macro_rules! impl_side {
    ($method:ident, $field:ident) => {
        fn $method<T>(mut self, value: T) -> Self
        where
            T: IntoPaddingValue,
        {
            self.layout_mut().padding.$field =
                value.into_padding_value().unwrap_or_default().into();
            self
        }
    };
}

pub trait Padding: Drawable {
    fn padding<T>(mut self, value: T) -> Self
    where
        T: IntoPadding,
    {
        self.layout_mut().padding = value.into_padding().unwrap_or_default().into();
        self
    }

    fn padding_x<T>(mut self, value: T) -> Self
    where
        T: IntoPaddingPair,
    {
        let (left, right) = value.into_padding_pair().unwrap_or_default();
        self.layout_mut().padding.left = left.into();
        self.layout_mut().padding.right = right.into();
        self
    }

    fn padding_y<T>(mut self, value: T) -> Self
    where
        T: IntoPaddingPair,
    {
        let (top, bottom) = value.into_padding_pair().unwrap_or_default();
        self.layout_mut().padding.top = top.into();
        self.layout_mut().padding.bottom = bottom.into();
        self
    }

    impl_side!(padding_top, top);
    impl_side!(padding_right, right);
    impl_side!(padding_bottom, bottom);
    impl_side!(padding_left, left);
}
