use crate::utils::FloatWriter;
use std::fmt::Write;

pub(super) trait Writer<T>: Sized + FloatWriter<T>
where
    T: Write,
{
    const SPACE: char = ' ';

    fn str(&mut self, str: &str) -> Result<&mut Self, std::fmt::Error> {
        self.out_mut().write_str(str)?;
        Ok(self)
    }

    fn char(&mut self, char: char) -> Result<&mut Self, std::fmt::Error> {
        self.out_mut().write_char(char)?;
        Ok(self)
    }

    fn space(&mut self) -> Result<&mut Self, std::fmt::Error> {
        self.char(Self::SPACE)
    }

    fn float(&mut self, value: f32) -> Result<&mut Self, std::fmt::Error> {
        self.float_precise(value, Self::FLOAT_SCALE)
    }

    fn float_precise(&mut self, value: f32, scale: f32) -> Result<&mut Self, std::fmt::Error> {
        self.write_float_precise(value, scale)?;
        Ok(self)
    }
}
