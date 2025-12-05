macro_rules! impl_background_methods {
    ($node_name:ident) => {
        impl $node_name {
            pub fn background<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoFill,
            {
                self.visual.background = value.into_fill().unwrap_or(crate::attributes::Fill::None);
                self
            }
        }
    };
}

pub(crate) use impl_background_methods;
