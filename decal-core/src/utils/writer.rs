use crate::utils::FloatWriter;
use std::fmt::Write;

pub(super) trait Writer<T>: Sized + FloatWriter<T>
where
    T: Write,
{
    const SPACE: char = ' ';

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

    fn space(&mut self) -> &mut Self {
        self.char(Self::SPACE)
    }

    fn float(&mut self, value: f32) -> &mut Self {
        self.float_precise(value, Self::FLOAT_SCALE)
    }

    fn float_precise(&mut self, value: f32, scale: f32) -> &mut Self {
        if self.result_mut().is_ok() {
            *self.result_mut() = self.write_float_precise(value, scale);
        }

        self
    }
}
