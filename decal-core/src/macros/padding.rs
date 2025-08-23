macro_rules! impl_padding_methods {
    ($node_name:ident) => {
        use crate::attributes::IntoPadding;

        macro_rules! impl_side {
            ($method:ident, $field:ident) => {
                pub fn $method<T>(&mut self, value: T) -> &mut Self
                where
                    T: Into<Option<crate::primitives::Length>>,
                {
                    self.style.padding.$field =
                        value.into().map_or(taffy::LengthPercentage::ZERO, |inner| {
                            inner.to_length_percentage()
                        });
                    self
                }
            };
        }

        impl $node_name {
            pub fn padding<T>(&mut self, value: T) -> &mut Self
            where
                T: IntoPadding,
            {
                self.style.padding = value
                    .into_padding()
                    .map_or(taffy::Rect::zero(), |inner| inner.to_style());
                self
            }

            impl_side!(padding_top, top);
            impl_side!(padding_right, right);
            impl_side!(padding_bottom, bottom);
            impl_side!(padding_left, left);
        }
    };
}

pub(crate) use impl_padding_methods;
