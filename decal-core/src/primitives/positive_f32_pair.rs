use crate::{
    macros::pf32,
    utils::FloatWriter,
};
use std::fmt::{
    Display,
    Formatter,
    Write,
};
use strict_num::PositiveF32;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct PositiveF32Pair(PositiveF32, PositiveF32);

impl PositiveF32Pair {
    pub(crate) fn is_zero(&self) -> bool {
        self.0 == PositiveF32::ZERO && self.1 == PositiveF32::ZERO
    }
}

impl Default for PositiveF32Pair {
    fn default() -> Self {
        PositiveF32Pair(PositiveF32::ZERO, PositiveF32::ZERO)
    }
}

impl From<f32> for PositiveF32Pair {
    fn from(value: f32) -> Self {
        let value = pf32!(value);
        PositiveF32Pair(value, value)
    }
}

impl From<(f32, f32)> for PositiveF32Pair {
    fn from((x, y): (f32, f32)) -> Self {
        PositiveF32Pair(pf32!(x), pf32!(y))
    }
}

impl Display for PositiveF32Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_float(self.0.get())?;

        if self.0 != self.1 {
            f.write_char(' ')?;
            f.write_float(self.1.get())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_zero() {
        let pair = PositiveF32Pair::default();
        assert!(pair.is_zero());
        assert_eq!(pair, PositiveF32Pair(PositiveF32::ZERO, PositiveF32::ZERO));
    }

    #[test]
    fn is_zero() {
        assert!(PositiveF32Pair::from(0.0).is_zero());
        assert!(!PositiveF32Pair::from((0.0, 1.0)).is_zero());
        assert!(!PositiveF32Pair::from((1.0, 0.0)).is_zero());
    }

    #[test]
    fn from_single_value() {
        assert_eq!(
            PositiveF32Pair::from(2.5),
            PositiveF32Pair(pf32!(2.5), pf32!(2.5))
        );
    }

    #[test]
    fn from_tuple() {
        assert_eq!(
            PositiveF32Pair::from((1.5, 2.5)),
            PositiveF32Pair(pf32!(1.5), pf32!(2.5))
        );
    }

    #[test]
    fn renders_single_value() {
        assert_eq!(PositiveF32Pair::from(3.5).to_string(), "3.5");
    }

    #[test]
    fn renders_tuple() {
        assert_eq!(PositiveF32Pair::from((3.2, 4.5)).to_string(), "3.2 4.5");
    }
}
