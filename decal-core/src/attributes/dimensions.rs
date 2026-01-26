use crate::primitives::{
    Length,
    Size,
};

type Dimensions = Size<Length>;

pub trait IntoDimensions {
    fn into_dimensions(self) -> Option<Dimensions>;
}

impl IntoDimensions for Option<Dimensions> {
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        self
    }
}

impl IntoDimensions for Dimensions {
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        Some(self)
    }
}

impl<T> IntoDimensions for T
where
    T: Into<Length> + Copy,
{
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        Some(Size {
            width: self.into(),
            height: self.into(),
        })
    }
}

impl<T> IntoDimensions for (T, T)
where
    T: Into<Length> + Copy,
{
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        Some(Size {
            width: self.0.into(),
            height: self.1.into(),
        })
    }
}

impl<T> IntoDimensions for [T; 2]
where
    T: Into<Length> + Copy,
{
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        IntoDimensions::into_dimensions((self[0], self[1]))
    }
}
