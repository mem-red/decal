use ryu::Buffer;
use std::fmt::{Display, Write};

pub(super) trait Writer<T>: Sized
where
    T: Write,
{
    const DEFAULT_FLOAT_SCALE: f32 = 10_000.0;
    const SPACE: char = ' ';

    fn out_mut(&mut self) -> &mut T;
    fn result_mut(&mut self) -> &mut std::fmt::Result;

    fn str(&mut self, str: &str) -> &mut Self {
        if self.result_mut().is_ok() {
            *self.result_mut() = self.out_mut().write_str(str);
        }

        self
    }

    fn char(&mut self, char: char) -> &mut Self {
        if self.result_mut().is_ok() {
            *self.result_mut() = self.out_mut().write_char(char);
        }

        self
    }

    fn fmt<D>(&mut self, value: D) -> &mut Self
    where
        D: Display,
    {
        if self.result_mut().is_ok() {
            *self.result_mut() = self.out_mut().write_fmt(format_args!("{value}"));
        }

        self
    }

    fn space(&mut self) -> &mut Self {
        self.char(Self::SPACE)
    }

    fn float(&mut self, value: f32) -> &mut Self {
        self.float_precise(value, Self::DEFAULT_FLOAT_SCALE)
    }

    fn float_precise(&mut self, mut value: f32, scale: f32) -> &mut Self {
        let mut buf = Buffer::new();
        value = (value * scale).round() / scale;

        if value.fract() == 0.0 {
            return self.fmt(value as i32);
        }

        self.str(buf.format_finite(value))
    }
}
