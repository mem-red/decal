macro_rules! impl_visibility_methods {
    ($node_name:ident) => {
        impl $node_name {
            pub fn visible(&mut self, value: bool) -> &mut Self
            {
                self.visual.visible = value;
                self
            }
        }
    };
}

pub(crate) use impl_visibility_methods;
