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

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Default)]
    struct Sink(String);

    impl FloatWriter<String> for Sink {
        fn out_mut(&mut self) -> &mut String {
            &mut self.0
        }
    }

    impl Writer<String> for Sink {}

    fn sink<F>(visit: F) -> String
    where
        F: FnOnce(&mut Sink) -> Result<&mut Sink, std::fmt::Error>,
    {
        let mut out = Sink::default();
        visit(&mut out).unwrap();
        out.0
    }

    #[test]
    fn writes_str() {
        assert_eq!(sink(|out| out.str("test")), "test");
    }

    #[test]
    fn writes_char() {
        assert_eq!(sink(|out| out.char('Z')), "Z");
    }

    #[test]
    fn writes_space() {
        assert_eq!(sink(|out| out.space()), " ");
    }

    #[test]
    fn writes_float() {
        assert_eq!(sink(|out| out.float(2.25)), "2.25");
    }
}
