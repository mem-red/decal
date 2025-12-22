use ryu::Buffer;
use std::fmt::Write;

pub(crate) trait FloatWriter<T>
where
    T: Write,
{
    const FLOAT_SCALE: f32 = 10_000.0;

    fn out_mut(&mut self) -> &mut T;

    fn write_float(&mut self, value: f32) -> std::fmt::Result
    where
        T: Write,
    {
        self.write_float_precise(value, Self::FLOAT_SCALE)
    }

    fn write_float_precise(&mut self, mut value: f32, scale: f32) -> std::fmt::Result
    where
        T: Write,
    {
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
