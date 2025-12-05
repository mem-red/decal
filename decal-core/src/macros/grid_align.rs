macro_rules! impl_grid_align_methods {
    ($node_name:ident) => {
        impl $node_name {
            pub fn justify_items<T>(&mut self, value: T) -> &mut Self
            where
                T: Into<Option<crate::attributes::JustifyItems>>,
            {
                self.layout.justify_items = value.into().map(|x| x.into());
                self
            }
        }
    };
}

pub(crate) use impl_grid_align_methods;
