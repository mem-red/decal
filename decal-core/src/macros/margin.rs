macro_rules! impl_margin_methods {
    ($node_name:ident) => {
        macro_rules! impl_side {
            ($method:ident, $field:ident) => {
                pub fn $method<T>(&mut self, value: T) -> &mut Self
                where
                    T: Into<Option<crate::primitives::Length>>,
                {
                    self.layout.margin.$field = value.into().unwrap_or_default().into();
                    self
                }
            };
        }

        impl $node_name {
            pub fn margin<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoMargin,
            {
                self.layout.margin = value.into_margin().unwrap_or_default().into();
                self
            }

            pub fn margin_x<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoMarginPair,
            {
                let (left, right) = value.into_margin_pair().unwrap_or_default();
                self.layout.margin.left = left.into();
                self.layout.margin.right = right.into();
                self
            }

            pub fn margin_y<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoMarginPair,
            {
                let (top, bottom) = value.into_margin_pair().unwrap_or_default();
                self.layout.margin.top = top.into();
                self.layout.margin.bottom = bottom.into();
                self
            }

            impl_side!(margin_top, top);
            impl_side!(margin_right, right);
            impl_side!(margin_bottom, bottom);
            impl_side!(margin_left, left);
        }
    };
}

pub(crate) use impl_margin_methods;
