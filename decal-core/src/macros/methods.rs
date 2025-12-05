macro_rules! impl_node_methods {
    ($node:ty, [$($method:ident),*]) => {
        $(
            paste::paste! {
                crate::macros::[<impl_ $method _methods>]!($node);
            }
        )+
    };
}

pub(crate) use impl_node_methods;
