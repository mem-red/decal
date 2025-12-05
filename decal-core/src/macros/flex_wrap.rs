macro_rules! impl_flex_wrap_methods {
    ($node_name:ident) => {
        impl $node_name {
            pub fn flex_wrap(&mut self, value: crate::attributes::FlexWrap) -> &mut Self {
                self.layout.flex_wrap = value.into();
                self
            }
        }
    };
}

pub(crate) use impl_flex_wrap_methods;
