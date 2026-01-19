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
