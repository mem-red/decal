use crate::primitives::Color;
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

    pub fn offset<T>(mut self, offset: T) -> Self
    where
        T: Into<f32>,
    {
        self.offset = NormalizedF32::new_clamped(offset.into());
        self
    }

    pub fn offset_pct<T>(self, offset: T) -> Self
    where
        T: Into<f32>,
    {
        self.offset(offset.into() / 100.0);
        self
    }

    pub fn color<T>(mut self, color: T) -> Self
    where
        T: Into<Color>,
    {
        self.color = color.into();
        self
    }

    pub fn opacity<T>(mut self, opacity: T) -> Self
    where
        T: Into<f32>,
    {
        self.opacity = NormalizedF32::new_clamped(opacity.into());
        self
    }
}

impl Display for Stop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<stop offset="{}" stop-color="{}""#,
            self.offset, self.color
        )?;

        if self.opacity != 1.0 {
            write!(f, r#" stop-opacity="{}""#, self.opacity)?;
        }

        write!(f, r#" />"#)?;

        Ok(())
    }
}

impl<T> From<(T, Color)> for Stop
where
    T: Into<f32>,
{
    fn from((offset, color): (T, Color)) -> Self {
        Stop::new().offset(offset).color(color)
    }
}

impl<T> From<(T, Color, T)> for Stop
where
    T: Into<f32>,
{
    fn from((offset, color, opacity): (T, Color, T)) -> Self {
        Stop::new().offset(offset).color(color).opacity(opacity)
    }
}
