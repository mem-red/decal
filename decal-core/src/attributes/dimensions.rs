use crate::primitives::{Length, Size};

#[derive(Debug, Clone, Copy, Default)]
pub struct Dimensions(pub Size<Length>);

impl Into<taffy::Size<taffy::Dimension>> for Dimensions {
    #[inline]
    fn into(self) -> taffy::Size<taffy::Dimension> {
        taffy::Size {
            width: self.width.into(),
            height: self.height.into(),
        }
    }
}

impl std::ops::Deref for Dimensions {
    type Target = Size<Length>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Dimensions {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
        Some(Dimensions(Size {
            width: self,
            height: self,
        }))
    }
}

impl IntoDimensions for [Length; 1] {
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        Some(Dimensions(Size {
            width: self[0],
            height: self[0],
        }))
    }
}

impl IntoDimensions for [Length; 2] {
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        Some(Dimensions(Size {
            width: self[0],
            height: self[1],
        }))
    }
}
