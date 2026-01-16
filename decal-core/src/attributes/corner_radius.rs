use crate::primitives::{Corner, Length};

type CornerRadii = Length<false, true>;
pub type CornerRadius = Corner<CornerRadii>;

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

impl<T> IntoCornerRadius for T
where
    T: Into<CornerRadii> + Copy,
{
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(Corner {
            top_left: self.into(),
            top_right: self.into(),
            bottom_right: self.into(),
            bottom_left: self.into(),
        })
    }
}

impl<T> IntoCornerRadius for [T; 1]
where
    T: Into<CornerRadii> + Copy,
{
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(Corner {
            top_left: self[0].into(),
            top_right: self[0].into(),
            bottom_right: self[0].into(),
            bottom_left: self[0].into(),
        })
    }
}

impl<T> IntoCornerRadius for [T; 2]
where
    T: Into<CornerRadii> + Copy,
{
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(Corner {
            top_left: self[0].into(),
            top_right: self[1].into(),
            bottom_right: self[0].into(),
            bottom_left: self[1].into(),
        })
    }
}

impl<T> IntoCornerRadius for [T; 3]
where
    T: Into<CornerRadii> + Copy,
{
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(Corner {
            top_left: self[0].into(),
            top_right: self[1].into(),
            bottom_right: self[2].into(),
            bottom_left: self[1].into(),
        })
    }
}

impl<T> IntoCornerRadius for [T; 4]
where
    T: Into<CornerRadii> + Copy,
{
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(Corner {
            top_left: self[0].into(),
            top_right: self[1].into(),
            bottom_right: self[2].into(),
            bottom_left: self[3].into(),
        })
    }
}
