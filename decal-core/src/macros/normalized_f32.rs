/// Creates a [`NormalizedF32`] value clamped to `[0.0, 1.0]`.
///
/// # Usage
/// - `nf32!(expr)`: Converts `expr` into a normalized value.
///
/// # Arguments
/// - `expr`: A numeric expression convertible into `f32`.
///
/// # Returns
/// - A [`NormalizedF32`] value.
///
/// [`NormalizedF32`]: strict_num::NormalizedF32
macro_rules! nf32 {
    ($expr:expr) => {
        strict_num::NormalizedF32::new_clamped($expr.into())
    };
}

pub(crate) use nf32;
