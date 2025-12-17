use super::Drawable;
use crate::primitives::Overflow;

macro_rules! impl_axis {
    ($method:ident, $field:ident) => {
        fn $method<T>(&mut self, value: T) -> &mut Self
        where
            T: Into<Option<crate::primitives::Overflow>>,
        {
            self.layout_mut().overflow.$field = value.into().unwrap_or_default().into();
            self
        }
    };
}

pub trait Clippable: Drawable {
    fn overflow<T>(&mut self, value: T) -> &mut Self
    where
        T: crate::attributes::IntoOverflow,
    {
        self.layout_mut().overflow = value.into_overflow().unwrap_or_default().into();
        self
    }

    impl_axis!(overflow_x, x);
    impl_axis!(overflow_y, y);

    //

    fn overflow_hidden(&mut self) -> &mut Self {
        self.overflow(Overflow::Hidden);
        self
    }

    fn overflow_visible(&mut self) -> &mut Self {
        self.overflow(Overflow::Visible);
        self
    }
}
