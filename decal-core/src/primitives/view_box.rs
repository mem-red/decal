use crate::{
    macros::ff32,
    utils::FloatWriter,
};
use std::fmt::{
    Display,
    Formatter,
    Write,
};
use strict_num::FiniteF32;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct ViewBox {
    min_x: FiniteF32,
    min_y: FiniteF32,
    width: FiniteF32,
    height: FiniteF32,
}

impl ViewBox {
    pub fn new(min_x: f32, min_y: f32, width: f32, height: f32) -> Self {
        Self {
            min_x: ff32!(min_x),
            min_y: ff32!(min_y),
            width: ff32!(width),
            height: ff32!(height),
        }
    }
}

impl Display for ViewBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_float(self.min_x.get())?;
        f.write_char(' ')?;
        f.write_float(self.min_y.get())?;
        f.write_char(' ')?;
        f.write_float(self.width.get())?;
        f.write_char(' ')?;
        f.write_float(self.height.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders() {
        assert_eq!(
            ViewBox::new(0.2, 0.3, 150.0, 250.0).to_string(),
            "0.2 0.3 150 250"
        );
    }
}
