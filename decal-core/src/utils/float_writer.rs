use ryu::Buffer;
use std::fmt::Write;

/// Utility trait for pretty-writing floating point values with controlled
/// precision.
pub(crate) trait FloatWriter<T>
where
    T: Write,
{
    const FLOAT_SCALE: f32 = 10_000.0;

    /// Returns a mutable reference to the underlying output writer.
    fn out_mut(&mut self) -> &mut T;

    /// Writes a float value using the default precision.
    ///
    /// # Arguments
    /// - `value`: The float value to write.
    fn write_float(&mut self, value: f32) -> std::fmt::Result
    where
        T: Write,
    {
        self.write_float_precise(value, Self::FLOAT_SCALE)
    }

    /// Writes a float value using a custom precision.
    ///
    /// Values are rounded to the nearest multiple of `1 / scale` before being
    /// written.
    ///
    /// # Arguments
    /// - `value`: The float value to write.
    /// - `scale`: Rounding scale.
    fn write_float_precise(&mut self, mut value: f32, scale: f32) -> std::fmt::Result
    where
        T: Write,
    {
        debug_assert_ne!(scale, 0.0);

        value = (value * scale).round() / scale;

        if value.fract() == 0.0 {
            return write!(self.out_mut(), "{}", value as i32);
        }

        let mut buf = Buffer::new();
        self.out_mut().write_str(buf.format_finite(value))
    }
}

impl<T> FloatWriter<T> for T
where
    T: Write,
{
    fn out_mut(&mut self) -> &mut T {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::str_sink;

    #[test]
    fn writes_integer_without_decimal() {
        assert_eq!(str_sink(|x| x.write_float(5.0)), "5");
    }

    #[test]
    fn writes_float() {
        assert_eq!(str_sink(|x| x.write_float(2.25)), "2.25");
    }

    #[test]
    fn rounds_to_default_scale() {
        assert_eq!(str_sink(|x| x.write_float(1.2345678)), "1.2346");
    }

    #[test]
    fn rounds_down() {
        assert_eq!(str_sink(|x| x.write_float(2.00004)), "2");
    }

    #[test]
    fn rounds_up() {
        assert_eq!(str_sink(|x| x.write_float(2.00005)), "2.0001");
    }

    #[test]
    fn writes_negative_floats() {
        assert_eq!(str_sink(|x| x.write_float(-2.25)), "-2.25");
    }

    #[test]
    fn tiny_value_rounds_to_zero() {
        assert_eq!(str_sink(|x| x.write_float(1e-5)), "0");
    }

    #[test]
    fn writes_float_with_custom_scale() {
        assert_eq!(str_sink(|x| x.write_float_precise(1.2345678, 10.0)), "1.2");
    }
}
