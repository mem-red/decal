// finite f32
macro_rules! ff32 {
    ($expr:expr) => {
        FiniteF32::new($expr).unwrap_or_default()
    };
    ($expr:expr, $default:expr) => {
        FiniteF32::new($expr).unwrap_or_else(|| FiniteF32::new($default).unwrap_or_default())
    };
}

pub(crate) use ff32;
