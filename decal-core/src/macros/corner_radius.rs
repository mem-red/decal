macro_rules! impl_corner_radius_methods {
    ($node_name:ident) => {
        macro_rules! impl_corner {
            ($method:ident, $field:ident) => {
                pub fn $method<T>(&mut self, value: T) -> &mut Self
                where
                    T: Into<Option<crate::primitives::Length>>,
                {
                    self.visual.corner_radius.$field = value.into().unwrap_or_default();
                    self
                }
            };
        }

        impl $node_name {
            pub fn corner_radius<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoCornerRadius,
            {
                self.visual.corner_radius = value.into_corner_radius().unwrap_or_default();
                self
            }

            impl_corner!(corner_top_left_radius, top_left);
            impl_corner!(corner_top_right_radius, top_right);
            impl_corner!(corner_bottom_right_radius, bottom_right);
            impl_corner!(corner_bottom_left_radius, bottom_left);
        }
    };
}

pub(crate) use impl_corner_radius_methods;
