use crate::prelude::Corner;
use crate::primitives::Length;

#[derive(Debug, Clone, Copy, Default)]
pub struct CornerRadius(pub Corner<Length>);

impl CornerRadius {
    pub fn is_zero(&self) -> bool {
        self.top_left.is_zero()
            && self.top_right.is_zero()
            && self.bottom_right.is_zero()
            && self.bottom_left.is_zero()
    }
}

impl std::ops::Deref for CornerRadius {
    type Target = Corner<Length>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CornerRadius {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait IntoCornerRadius {
    fn into_corner_radius(self) -> Option<CornerRadius>;
}

impl IntoCornerRadius for Option<CornerRadius> {
    fn into_corner_radius(self) -> Option<CornerRadius> {
        self
    }
}

impl IntoCornerRadius for CornerRadius {
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(self)
    }
}

impl IntoCornerRadius for Length {
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(CornerRadius(Corner {
            top_left: self,
            top_right: self,
            bottom_right: self,
            bottom_left: self,
        }))
    }
}

impl IntoCornerRadius for [Length; 1] {
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(CornerRadius(Corner {
            top_left: self[0],
            top_right: self[0],
            bottom_right: self[0],
            bottom_left: self[0],
        }))
    }
}

impl IntoCornerRadius for [Length; 2] {
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(CornerRadius(Corner {
            top_left: self[0],
            top_right: self[1],
            bottom_right: self[0],
            bottom_left: self[1],
        }))
    }
}

impl IntoCornerRadius for [Length; 3] {
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(CornerRadius(Corner {
            top_left: self[0],
            top_right: self[1],
            bottom_right: self[2],
            bottom_left: self[1],
        }))
    }
}

impl IntoCornerRadius for [Length; 4] {
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(CornerRadius(Corner {
            top_left: self[0],
            top_right: self[1],
            bottom_right: self[2],
            bottom_left: self[3],
        }))
    }
}
