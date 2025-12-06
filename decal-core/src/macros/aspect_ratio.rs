macro_rules! impl_aspect_ratio_methods {
    ($node_name:ident) => {
        impl $node_name {
            pub fn aspect_ratio<T>(&mut self, value: T) -> &mut Self
            where
                T: Into<Option<f32>>,
            {
                self.layout.aspect_ratio = value.into();
                self
            }
        }
    };
}

pub(crate) use impl_aspect_ratio_methods;
