use super::Drawable;
use crate::{
    attributes::IntoOverflow,
    primitives::Overflow,
};

macro_rules! impl_axis {
    ($method:ident, $field:ident) => {
        fn $method<T>(mut self, value: T) -> Self
        where
            T: Into<Option<Overflow>>,
        {
            self.layout_mut().overflow.$field = value.into().unwrap_or_default().into();
            self
        }
    };
}

pub trait Clippable: Drawable {
    fn overflow<T>(mut self, value: T) -> Self
    where
        T: IntoOverflow,
    {
        self.layout_mut().overflow = value.into_overflow().unwrap_or_default().into();
        self
    }

    impl_axis!(overflow_x, x);
    impl_axis!(overflow_y, y);

    //

    fn overflow_hidden(self) -> Self {
        self.overflow(Overflow::Hidden)
    }

    fn overflow_visible(self) -> Self {
        self.overflow(Overflow::Visible)
    }
}
