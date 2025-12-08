macro_rules! impl_transform_methods {
    ($node_name:ident) => {
        impl $node_name {
            pub fn transform<T>(&mut self, value: T) -> &mut Self
            where
                T: Into<Option<crate::primitives::Transform>>,
            {
                self.visual.transform = value.into().unwrap_or_default();
                self
            }
        }
    };
}

pub(crate) use impl_transform_methods;
