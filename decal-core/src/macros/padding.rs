macro_rules! impl_padding_methods {
    ($node_name:ident) => {
        macro_rules! impl_side {
            ($method:ident, $field:ident) => {
                pub fn $method<T>(&mut self, value: T) -> &mut Self
                where
                    T: Into<Option<crate::primitives::Length>>,
                {
                    self.layout.padding.$field = value.into().unwrap_or_default().into();
                    self
                }
            };
        }

        impl $node_name {
            pub fn padding<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoPadding,
            {
                self.layout.padding = value.into_padding().unwrap_or_default().into();
                self
            }

            pub fn padding_x<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoPaddingPair,
            {
                let (left, right) = value.into_padding_pair().unwrap_or_default();
                self.layout.padding.left = left.into();
                self.layout.padding.right = right.into();
                self
            }

            pub fn padding_y<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoPaddingPair,
            {
                let (top, bottom) = value.into_padding_pair().unwrap_or_default();
                self.layout.padding.top = top.into();
                self.layout.padding.bottom = bottom.into();
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
