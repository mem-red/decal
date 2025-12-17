use crate::primitives::{Length, Size};

type Gap = Size<Length>;

pub trait IntoGap {
    fn into_gap(self) -> Option<Gap>;
}

impl IntoGap for Option<Gap> {
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        self
    }
}

impl IntoGap for Gap {
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        Some(self)
    }
}

impl IntoGap for Length {
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        Some(Size {
            width: self,
            height: self,
        })
    }
}

impl IntoGap for [Length; 1] {
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        Some(Size {
            width: self[0],
            height: self[0],
        })
    }
}

impl IntoGap for [Length; 2] {
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        Some(Size {
            width: self[0],
            height: self[1],
        })
    }
}
