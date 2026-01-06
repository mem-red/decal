use crate::macros::nf32;
use crate::primitives::Color;
use crate::utils::ElementWriter;
use std::fmt::{Display, Formatter};
use strict_num::NormalizedF32;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Stop {
    offset: NormalizedF32,
    color: Color,
    opacity: NormalizedF32,
}

impl Default for Stop {
    fn default() -> Self {
        Stop {
            offset: NormalizedF32::ZERO,
            color: Color::default(),
            opacity: NormalizedF32::ONE,
        }
    }
}

impl Stop {
    pub fn new() -> Self {
        Stop::default()
    }

    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = nf32!(offset);
        self
    }

    pub fn offset_pct(self, offset: f32) -> Self {
        self.offset(offset / 100.0);
        self
    }

    pub fn color<T>(mut self, color: T) -> Self
    where
        T: Into<Color>,
    {
        self.color = color.into();
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = nf32!(opacity);
        self
    }
}

impl Display for Stop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "stop")?
            .attr("stop-color", (self.color,))?
            .attr("offset", self.offset)?
            .attr_if(
                "stop-opacity",
                self.opacity,
                self.opacity != NormalizedF32::ONE,
            )?
            .close()
    }
}

impl From<(f32, Color)> for Stop {
    fn from((offset, color): (f32, Color)) -> Self {
        Stop::new().offset(offset).color(color)
    }
}

impl From<(Color, f32)> for Stop {
    fn from((color, opacity): (Color, f32)) -> Self {
        Stop::new().color(color).opacity(opacity)
    }
}

impl From<(f32, Color, f32)> for Stop {
    fn from((offset, color, opacity): (f32, Color, f32)) -> Self {
        Stop::new().offset(offset).color(color).opacity(opacity)
    }
}
