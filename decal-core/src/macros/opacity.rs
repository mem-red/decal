macro_rules! impl_opacity_methods {
    ($node_name:ident) => {
        impl $node_name {
            pub fn opacity(&mut self, value: f32) -> &mut Self {
                debug_assert!(value >= 0.0 && value <= 1.0);
                self.visual.opacity = value.clamp(0.0, 1.0);
                self
            }
        }
    };
}

pub(crate) use impl_opacity_methods;
