macro_rules! impl_gap_methods {
    ($node_name:ident) => {
        impl $node_name {
            pub fn gap<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoGap,
            {
                self.layout.gap = value.into_gap().unwrap_or_default().into();
                self
            }
        }
    };
}

pub(crate) use impl_gap_methods;
