use crate::primitives::{Length, Size};

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

impl IntoDimensions for Length {
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        Some(Size {
            width: self,
            height: self,
        })
    }
}

impl IntoDimensions for [Length; 1] {
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        Some(Size {
            width: self[0],
            height: self[0],
        })
    }
}

impl IntoDimensions for [Length; 2] {
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        Some(Size {
            width: self[0],
            height: self[1],
        })
    }
}
