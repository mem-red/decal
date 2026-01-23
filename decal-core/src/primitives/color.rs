use crate::utils::FloatWriter;
use color::{
    AlphaColor,
    Srgb,
    parse_color,
};
use std::fmt::{
    Display,
    Write,
};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        debug_assert!(a >= 0.0 && a <= 1.0);
        Self {
            r,
            g,
            b,
            a: (a.clamp(0.0, 1.0) * 255.0).round() as u8,
        }
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
        if self.a == 255 {
            f.write_fmt(format_args!("rgb({},{},{})", self.r, self.g, self.b))
        } else {
            f.write_fmt(format_args!("rgba({},{},{},", self.r, self.g, self.b))?;
            f.write_float(self.a as f32 / 255.0)?;
            f.write_char(')')
        }
    }
}

impl From<color::Rgba8> for Color {
    #[inline]
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
    #[inline]
    fn from(color: color::DynamicColor) -> Self {
        let srgb: AlphaColor<Srgb> = color.to_alpha_color();
        srgb.to_rgba8().into()
    }
}

impl From<[u8; 3]> for Color {
    #[inline]
    fn from(value: [u8; 3]) -> Self {
        Self::rgb(value[0], value[1], value[2])
    }
}

impl From<([u8; 3], f32)> for Color {
    #[inline]
    fn from(value: ([u8; 3], f32)) -> Self {
        let [r, g, b] = value.0;
        Self::rgba(r, g, b, value.1)
    }
}

impl From<&str> for Color {
    #[inline]
    fn from(value: &str) -> Self {
        Self::parse(value)
    }
}

impl From<String> for Color {
    #[inline]
    fn from(value: String) -> Self {
        Self::parse(value.as_str())
    }
}

pub(super) mod helpers {
    use super::Color;

    pub trait IntoRgb {
        fn into_rgb(self) -> Color;
    }

    impl IntoRgb for u32 {
        fn into_rgb(self) -> Color {
            let [_, r, g, b] = self.to_be_bytes();
            Color::rgb(r, g, b)
        }
    }

    impl IntoRgb for (u8, u8, u8) {
        fn into_rgb(self) -> Color {
            Color::rgb(self.0, self.1, self.2)
        }
    }

    pub fn rgb<T>(value: T) -> Color
    where
        T: IntoRgb,
    {
        value.into_rgb()
    }

    //

    pub trait IntoRgba {
        fn into_rgba(self) -> Color;
    }

    impl IntoRgba for u32 {
        fn into_rgba(self) -> Color {
            let [r, g, b, a] = self.to_be_bytes();
            Color { r, g, b, a }
        }
    }

    impl IntoRgba for (u8, u8, u8, f32) {
        fn into_rgba(self) -> Color {
            Color::rgba(self.0, self.1, self.2, self.3)
        }
    }

    pub fn rgba<T>(value: T) -> Color
    where
        T: IntoRgba,
    {
        value.into_rgba()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        helpers::*,
        *,
    };

    #[test]
    fn rgb_constructor() {
        assert_eq!(
            Color::rgb(1, 2, 3),
            Color {
                r: 1,
                g: 2,
                b: 3,
                a: 255
            }
        );
    }

    #[test]
    fn rgba_constructor() {
        assert_eq!(
            Color::rgba(1, 2, 3, 0.5),
            Color {
                r: 1,
                g: 2,
                b: 3,
                a: 128
            }
        );
    }

    #[test]
    fn defaults_to_opaque_black() {
        assert_eq!(Color::default(), Color::rgb(0, 0, 0));
    }

    #[test]
    fn parses_rgb_string() {
        assert_eq!(Color::parse("rgb(1,2,3)"), Color::rgb(1, 2, 3));
    }

    #[test]
    fn parses_rgba_string() {
        assert_eq!(Color::parse("rgb(1,2,3,50%)"), Color::rgba(1, 2, 3, 0.5));
    }

    #[test]
    fn handles_invalid_string() {
        assert!(Color::try_parse("test").is_none());
    }

    #[test]
    fn parse_fallbacks_to_default() {
        assert_eq!(Color::parse("test"), Color::default());
    }

    #[test]
    fn renders() {
        assert_eq!(Color::rgb(1, 2, 3).to_string(), "rgb(1,2,3)");
        assert_eq!(Color::rgba(1, 2, 3, 0.5).to_string(), "rgba(1,2,3,0.502)");
        assert_eq!(Color::rgba(1, 2, 3, 1.0).to_string(), "rgb(1,2,3)");
    }

    //

    #[test]
    fn from_u8_array() {
        assert_eq!(Color::from([1, 2, 3]), Color::rgb(1, 2, 3));
    }

    #[test]
    fn from_rgb_tuple_with_alpha() {
        assert_eq!(Color::from(([1, 2, 3], 0.5)), Color::rgba(1, 2, 3, 0.5));
    }

    // helpers

    #[test]
    fn from_hex_helper() {
        assert_eq!(rgb(0x0a0b0c), Color::rgb(10, 11, 12));
        assert_eq!(rgba(0x0a0b0c80), Color::rgba(10, 11, 12, 0.5));
    }

    #[test]
    fn from_tuple_helper() {
        assert_eq!(rgb((1, 2, 3)), Color::rgb(1, 2, 3));
        assert_eq!(rgba((1, 2, 3, 0.5)), Color::rgba(1, 2, 3, 0.5));
    }
}
