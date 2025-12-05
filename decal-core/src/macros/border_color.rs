macro_rules! impl_border_color_methods {
    ($node_name:ident) => {
        impl $node_name {
            pub fn border_color<T>(&mut self, value: T) -> &mut Self
            where
                T: crate::attributes::IntoFill,
            {
                self.visual.border_color =
                    value.into_fill().unwrap_or(crate::attributes::Fill::None);
                self
            }
        }
    };
}

pub(crate) use impl_border_color_methods;
