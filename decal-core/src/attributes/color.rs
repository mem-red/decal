use color::{AlphaColor, Srgb, parse_color};
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r,
            g,
            b,
            a: ((a / 100.0).clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn parse(value: &str) -> Self {
        Self::try_parse(value).unwrap_or_default()
    }

    pub fn try_parse(value: &str) -> Option<Self> {
        let color = parse_color(value).ok()?;
        Some(color.into())
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::rgb(0, 0, 0)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "rgba({},{},{},{:.2})",
                self.r,
                self.g,
                self.b,
                self.a as f32 / 255.0
            )
        )
    }
}

impl From<color::Rgba8> for Color {
    fn from(value: color::Rgba8) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

impl From<color::DynamicColor> for Color {
    fn from(color: color::DynamicColor) -> Self {
        let srgb: AlphaColor<Srgb> = color.to_alpha_color();
        srgb.to_rgba8().into()
    }
}
