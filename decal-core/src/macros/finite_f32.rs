// finite f32
macro_rules! ff32 {
    ($expr:expr) => {
        strict_num::FiniteF32::new($expr.into()).unwrap_or_default()
    };
    ($expr:expr, $default:expr) => {
        strict_num::FiniteF32::new($expr.into())
            .unwrap_or_else(|| strict_num::FiniteF32::new($default.into()).unwrap_or_default())
    };
}

pub(crate) use ff32;
