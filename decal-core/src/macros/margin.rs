macro_rules! impl_margin_methods {
    ($node_name:ident) => {
        use crate::attributes::IntoMargin;

        macro_rules! impl_side {
            ($method:ident, $field:ident) => {
                pub fn $method<T>(&mut self, value: T) -> &mut Self
                where
                    T: Into<Option<crate::primitives::Length>>,
                {
                    self.style.margin.$field = value
                        .into()
                        .map_or(taffy::LengthPercentageAuto::ZERO, |inner| {
                            inner.to_length_percentage_auto()
                        });
                    self
                }
            };
        }

        impl $node_name {
            pub fn margin<T>(&mut self, value: T) -> &mut Self
            where
                T: IntoMargin,
            {
                self.style.margin = value
                    .into_margin()
                    .map_or(taffy::Rect::zero(), |inner| inner.to_style());
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
