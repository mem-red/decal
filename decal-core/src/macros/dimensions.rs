macro_rules! impl_dimensions_methods {
    ($node_name:ident) => {
        macro_rules! impl_dimension {
            ($method:ident, $taffy_method:ident, $taffy_field:ident) => {
                pub fn $method<T>(&mut self, value: T) -> &mut Self
                where
                    T: Into<Option<crate::primitives::Length>>,
                {
                    self.layout.$taffy_method.$taffy_field = value
                        .into()
                        .map_or(taffy::Dimension::auto(), |inner| inner.into());
                    self
                }
            };
        }

        impl $node_name {
            pub fn size<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoDimensions,
            {
                self.layout.size = value
                    .into_dimensions()
                    .map_or(taffy::Size::auto(), |inner| inner.into());
                self
            }

            pub fn min_size<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoDimensions,
            {
                self.layout.min_size = value
                    .into_dimensions()
                    .map_or(taffy::Size::auto(), |inner| inner.into());
                self
            }

            pub fn max_size<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoDimensions,
            {
                self.layout.max_size = value
                    .into_dimensions()
                    .map_or(taffy::Size::auto(), |inner| inner.into());
                self
            }

            impl_dimension!(width, size, width);
            impl_dimension!(height, size, height);
            //
            impl_dimension!(min_width, min_size, width);
            impl_dimension!(min_height, min_size, height);
            //
            impl_dimension!(max_width, max_size, width);
            impl_dimension!(max_height, max_size, height);
        }
    };
}

pub(crate) use impl_dimensions_methods;
