use crate::primitives::{Length, Size};

#[derive(Debug, Clone, Copy, Default)]
pub struct Gap(pub Size<Length>);

impl Into<taffy::Size<taffy::LengthPercentage>> for Gap {
    #[inline]
    fn into(self) -> taffy::Size<taffy::LengthPercentage> {
        taffy::Size {
            width: self.width.into(),
            height: self.height.into(),
        }
    }
}

impl std::ops::Deref for Gap {
    type Target = Size<Length>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Gap {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
        Some(Gap(Size {
            width: self,
            height: self,
        }))
    }
}

impl IntoGap for [Length; 1] {
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        Some(Gap(Size {
            width: self[0],
            height: self[0],
        }))
    }
}

impl IntoGap for [Length; 2] {
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        Some(Gap(Size {
            width: self[0],
            height: self[1],
        }))
    }
}
