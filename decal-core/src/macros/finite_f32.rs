/// Creates a [`FiniteF32`] value with fallback handling for non-finite inputs.
///
/// # Usage
/// - `ff32!(expr)`: Converts `expr` into a finite value, falling back to the
///   default finite value on failure.
/// - `ff32!(expr, default)`: Converts `expr` into a finite value, falling back
///   to `default` if `expr` is non-finite.
///
/// # Arguments
/// - `expr`: A numeric expression convertible into `f32`.
/// - `default`: A fallback numeric expression convertible into `f32` when
///   provided.
///
/// # Returns
/// - A [`FiniteF32`] value.
///
/// [`FiniteF32`]: strict_num::FiniteF32
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
