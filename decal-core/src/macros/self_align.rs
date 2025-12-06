macro_rules! impl_self_align_methods {
    ($node_name:ident) => {
        impl $node_name {
            pub fn align_self<T>(&mut self, value: T) -> &mut Self
            where
                T: Into<Option<crate::attributes::AlignSelf>>,
            {
                self.layout.align_self = value.into().map(|x| x.into());
                self
            }

            pub fn justify_self<T>(&mut self, value: T) -> &mut Self
            where
                T: Into<Option<crate::attributes::JustifySelf>>,
            {
                self.layout.justify_self = value.into().map(|x| x.into());
                self
            }

            pub fn flex_basis<T>(&mut self, value: T) -> &mut Self
            where
                T: Into<Option<crate::primitives::Length>>,
            {
                self.layout.flex_basis = value
                    .into()
                    .map(|x| x.into())
                    .unwrap_or(taffy::Style::DEFAULT.flex_basis);
                self
            }

            pub fn flex_grow<T>(&mut self, value: T) -> &mut Self
            where
                T: Into<Option<f32>>,
            {
                self.layout.flex_grow = value.into().unwrap_or(taffy::Style::DEFAULT.flex_grow);
                self
            }

            pub fn flex_shrink<T>(&mut self, value: T) -> &mut Self
            where
                T: Into<Option<f32>>,
            {
                self.layout.flex_shrink = value.into().unwrap_or(taffy::Style::DEFAULT.flex_shrink);
                self
            }
        }
    };
}

pub(crate) use impl_self_align_methods;
