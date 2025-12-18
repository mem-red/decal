use crate::primitives::{Color, Paint};

pub trait IntoPaint {
    fn into_paint(self) -> Option<Paint>;
}

impl IntoPaint for Option<Paint> {
    #[inline]
    fn into_paint(self) -> Option<Paint> {
        self
    }
}

impl IntoPaint for Paint {
    #[inline]
    fn into_paint(self) -> Option<Paint> {
        Some(self)
    }
}

impl IntoPaint for Color {
    #[inline]
    fn into_paint(self) -> Option<Paint> {
        Some(Paint::Color(self))
    }
}

impl IntoPaint for f32 {
    #[inline]
    fn into_paint(self) -> Option<Paint> {
        Some(Paint::Color(Color::rgba(0, 0, 0, self)))
    }
}

impl IntoPaint for f64 {
    #[inline]
    fn into_paint(self) -> Option<Paint> {
        Some(Paint::Color(Color::rgba(0, 0, 0, self as f32)))
    }
}

impl IntoPaint for &str {
    #[inline]
    fn into_paint(self) -> Option<Paint> {
        Some(Paint::Color(Color::parse(self)))
    }
}

impl IntoPaint for String {
    #[inline]
    fn into_paint(self) -> Option<Paint> {
        Some(Paint::Color(Color::parse(self.as_str())))
    }
}

impl<T> IntoPaint for [T; 1]
where
    T: Into<f64> + Copy,
{
    #[inline]
    fn into_paint(self) -> Option<Paint> {
        Some(Paint::Color(Color::rgba(0, 0, 0, self[0].into() as f32)))
    }
}

impl<T> IntoPaint for [T; 3]
where
    T: Into<u8> + Copy,
{
    #[inline]
    fn into_paint(self) -> Option<Paint> {
        Some(Paint::Color(Color::rgb(
            self[0].into(),
            self[1].into(),
            self[2].into(),
        )))
    }
}

impl<T> IntoPaint for [T; 4]
where
    T: Into<f64> + Copy,
{
    #[inline]
    fn into_paint(self) -> Option<Paint> {
        Some(Paint::Color(Color::rgba(
            self[0].into().clamp(0.0, 255.0) as u8,
            self[1].into().clamp(0.0, 255.0) as u8,
            self[2].into().clamp(0.0, 255.0) as u8,
            self[3].into() as f32,
        )))
    }
}
