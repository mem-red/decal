macro_rules! impl_position_methods {
    ($node_name:ident) => {
        impl $node_name {
            pub fn position(&mut self, value: crate::attributes::Position) -> &mut Self {
                self.layout.position = value.into();
                self
            }
        }
    };
}

pub(crate) use impl_position_methods;
