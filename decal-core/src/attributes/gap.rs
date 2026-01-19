use crate::primitives::{
    Length,
    Size,
};

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

impl<T> IntoGap for T
where
    T: Into<GapSize> + Copy,
{
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        Some(Size {
            width: self.into(),
            height: self.into(),
        })
    }
}

impl<T> IntoGap for [T; 1]
where
    T: Into<GapSize> + Copy,
{
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        Some(Size {
            width: self[0].into(),
            height: self[0].into(),
        })
    }
}

impl<T> IntoGap for [T; 2]
where
    T: Into<GapSize> + Copy,
{
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        Some(Size {
            width: self[0].into(),
            height: self[1].into(),
        })
    }
}
