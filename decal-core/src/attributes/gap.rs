use crate::primitives::{Length, Size};

type GapSize = Length<false, true>;
type Gap = Size<GapSize>;

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

impl IntoGap for GapSize {
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        Some(Size {
            width: self,
            height: self,
        })
    }
}

impl IntoGap for [GapSize; 1] {
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        Some(Size {
            width: self[0],
            height: self[0],
        })
    }
}

impl IntoGap for [GapSize; 2] {
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        Some(Size {
            width: self[0],
            height: self[1],
        })
    }
}
