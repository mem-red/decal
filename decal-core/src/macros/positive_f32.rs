// positive f32
macro_rules! pf32 {
    ($expr:expr) => {
        strict_num::PositiveF32::new($expr.into()).unwrap_or_default()
    };
    ($expr:expr, $default:expr) => {
        strict_num::PositiveF32::new($expr.into())
            .unwrap_or_else(|| strict_num::PositiveF32::new($default.into()).unwrap_or_default())
    };
}

pub(crate) use pf32;
