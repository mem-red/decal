// normalized f32
macro_rules! nf32 {
    ($expr:expr) => {
        strict_num::NormalizedF32::new_clamped($expr.into())
    };
}

pub(crate) use nf32;
