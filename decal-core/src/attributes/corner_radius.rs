use crate::capabilities::{CornerRadii, CornerRadius};
use crate::primitives::Corner;

pub trait IntoCornerRadius {
    fn into_corner_radius(self) -> Option<CornerRadius>;
}

impl IntoCornerRadius for Option<CornerRadius> {
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        self
    }
}

impl IntoCornerRadius for CornerRadius {
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(self)
    }
}

impl IntoCornerRadius for CornerRadii {
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(Corner {
            top_left: self,
            top_right: self,
            bottom_right: self,
            bottom_left: self,
        })
    }
}

impl IntoCornerRadius for [CornerRadii; 1] {
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(Corner {
            top_left: self[0],
            top_right: self[0],
            bottom_right: self[0],
            bottom_left: self[0],
        })
    }
}

impl IntoCornerRadius for [CornerRadii; 2] {
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(Corner {
            top_left: self[0],
            top_right: self[1],
            bottom_right: self[0],
            bottom_left: self[1],
        })
    }
}

impl IntoCornerRadius for [CornerRadii; 3] {
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(Corner {
            top_left: self[0],
            top_right: self[1],
            bottom_right: self[2],
            bottom_left: self[1],
        })
    }
}

impl IntoCornerRadius for [CornerRadii; 4] {
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(Corner {
            top_left: self[0],
            top_right: self[1],
            bottom_right: self[2],
            bottom_left: self[3],
        })
    }
}
