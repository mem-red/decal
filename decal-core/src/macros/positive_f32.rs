/// Creates a [`PositiveF32`] value with fallback handling for non-positive
/// inputs.
///
/// # Usage
/// - `ff32!(expr)`: Converts `expr` into a positive value, falling back to the
///   default positive value on failure.
/// - `ff32!(expr, default)`: Converts `expr` into a positive value, falling
///   back to `default` if `expr` is non-positive.
///
/// # Arguments
/// - `expr`: A numeric expression convertible into `f32`.
/// - `default`: A fallback numeric expression convertible into `f32` when
///   provided.
///
/// # Returns
/// - A [`PositiveF32`] value.
///
/// [`PositiveF32`]: strict_num::PositiveF32
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
