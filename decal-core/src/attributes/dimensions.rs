use crate::primitives::{Length, Size};

pub trait IntoDimension {
    fn into_dimension(self) -> Option<Length>;
}

impl IntoDimension for Option<Length> {
    #[inline]
    fn into_dimension(self) -> Option<Length> {
        self
    }
}

impl<T> IntoDimension for T
where
    T: Into<Length> + Copy,
{
    #[inline]
    fn into_dimension(self) -> Option<Length> {
        Some(self.into())
    }
}

//

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

impl<T> IntoDimensions for [T; 1]
where
    T: Into<Length> + Copy,
{
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        Some(Size {
            width: self[0].into(),
            height: self[0].into(),
        })
    }
}

impl<T> IntoDimensions for [T; 2]
where
    T: Into<Length> + Copy,
{
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        Some(Size {
            width: self[0].into(),
            height: self[1].into(),
        })
    }
}
