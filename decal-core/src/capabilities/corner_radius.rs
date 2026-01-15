use super::Drawable;
use crate::attributes::{IntoCornerRadii, IntoCornerRadius};

macro_rules! impl_corner {
    ($method:ident, $field:ident) => {
        fn $method<T>(mut self, value: T) -> Self
        where
            T: IntoCornerRadii,
        {
            self.visual_mut().corner_radius.$field = value.into_corner_radii().unwrap_or_default();
            self
        }
    };
}

pub trait RoundedCorners: Drawable {
    fn corner_radius<T>(mut self, value: T) -> Self
    where
        T: IntoCornerRadius,
    {
        self.visual_mut().corner_radius = value.into_corner_radius().unwrap_or_default();
        self
    }

    impl_corner!(corner_top_left_radius, top_left);
    impl_corner!(corner_top_right_radius, top_right);
    impl_corner!(corner_bottom_right_radius, bottom_right);
    impl_corner!(corner_bottom_left_radius, bottom_left);
}
