macro_rules! impl_container_align_methods {
    ($node_name:ident) => {
        impl $node_name {
            pub fn align_items<T>(&mut self, value: T) -> &mut Self
            where
                T: Into<Option<crate::attributes::AlignItems>>,
            {
                self.layout.align_items = value.into().map(|x| x.into());
                self
            }

            pub fn align_content<T>(&mut self, value: T) -> &mut Self
            where
                T: Into<Option<crate::attributes::AlignContent>>,
            {
                self.layout.align_content = value.into().map(|x| x.into());
                self
            }

            pub fn justify_content<T>(&mut self, value: T) -> &mut Self
            where
                T: Into<Option<crate::attributes::JustifyContent>>,
            {
                self.layout.justify_content = value.into().map(|x| x.into());
                self
            }
        }
    };
}

pub(crate) use impl_container_align_methods;
