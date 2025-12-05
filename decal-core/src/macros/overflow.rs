macro_rules! impl_overflow_methods {
    ($node_name:ident) => {
        macro_rules! impl_axis {
            ($method:ident, $field:ident) => {
                pub fn $method<T>(&mut self, value: T) -> &mut Self
                where
                    T: Into<Option<crate::primitives::Overflow>>,
                {
                    self.layout.overflow.$field = value.into().unwrap_or_default().into();
                    self
                }
            };
        }

        impl $node_name {
            pub fn overflow<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoOverflow,
            {
                self.layout.overflow = value.into_overflow().unwrap_or_default().into();
                self
            }

            impl_axis!(overflow_x, x);
            impl_axis!(overflow_y, y);
        }
    };
}

pub(crate) use impl_overflow_methods;
