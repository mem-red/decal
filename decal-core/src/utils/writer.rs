use crate::utils::FloatWriter;
use std::fmt::Write;

/// Utility trait providing fluent write helpers on top of [`FloatWriter`].
pub(super) trait Writer<T>: Sized + FloatWriter<T>
where
    T: Write,
{
    const SPACE: char = ' ';

    /// Writes a string slice to the underlying output.
    ///
    /// # Arguments
    /// - `str`: The string to write.
    ///
    /// # Returns
    /// - `Ok(&mut Self)` for chaining.
    fn str(&mut self, str: &str) -> Result<&mut Self, std::fmt::Error> {
        self.out_mut().write_str(str)?;
        Ok(self)
    }

    /// Writes a single character to the underlying output.
    ///
    /// # Arguments
    /// - `char`: The character to write.
    ///
    /// # Returns
    /// - `Ok(&mut Self)` for chaining.
    fn char(&mut self, char: char) -> Result<&mut Self, std::fmt::Error> {
        self.out_mut().write_char(char)?;
        Ok(self)
    }

    /// Writes a single space character.
    ///
    /// # Returns
    /// - `Ok(&mut Self)` for chaining.
    fn space(&mut self) -> Result<&mut Self, std::fmt::Error> {
        self.char(Self::SPACE)
    }

    /// Writes a float value using the default precision.
    ///
    /// # Arguments
    /// - `value`: The float value to write.
    ///
    /// # Returns
    /// - `Ok(&mut Self)` for chaining.
    fn float(&mut self, value: f32) -> Result<&mut Self, std::fmt::Error> {
        self.float_precise(value, Self::FLOAT_SCALE)
    }

    /// Writes a float value with custom precision.
    ///
    /// # Arguments
    /// - `value`: The float value to write.
    /// - `scale`: The rounding scale.
    ///
    /// # Returns
    /// - `Ok(&mut Self)` for chaining.
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
