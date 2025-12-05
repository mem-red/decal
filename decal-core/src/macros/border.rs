macro_rules! impl_border_methods {
    ($node_name:ident) => {
        macro_rules! impl_side {
            ($method:ident, $field:ident) => {
                pub fn $method<T>(&mut self, value: T) -> &mut Self
                where
                    T: Into<Option<crate::primitives::Length>>,
                {
                    self.layout.border.$field = value.into().unwrap_or_default().into();
                    self
                }
            };
        }

        impl $node_name {
            pub fn border<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoBorder,
            {
                self.layout.border = value.into_border().unwrap_or_default().into();
                self
            }

            impl_side!(border_top, top);
            impl_side!(border_right, right);
            impl_side!(border_bottom, bottom);
            impl_side!(border_left, left);
        }
    };
}

pub(crate) use impl_border_methods;
