use crate::attributes::Color;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default)]
pub enum Fill {
    #[default]
    None,
    Color(Color),
    Gradient,
    Pattern,
    Image,
}

impl Display for Fill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Fill::None => "none".into(),
                Fill::Color(color) => color.to_string(),
                Fill::Gradient => "".into(), // TODO
                Fill::Pattern => "".into(),  // TODO
                Fill::Image => "".into(),    // TODO
            }
        )
    }
}

pub trait IntoFill {
    fn into_fill(self) -> Option<Fill>;
}

impl IntoFill for Option<Fill> {
    fn into_fill(self) -> Option<Fill> {
        self
    }
}

impl IntoFill for Fill {
    fn into_fill(self) -> Option<Fill> {
        Some(self)
    }
}

impl IntoFill for f32 {
    fn into_fill(self) -> Option<Fill> {
        Some(Fill::Color(Color::new(0, 0, 0, self)))
    }
}

impl IntoFill for f64 {
    fn into_fill(self) -> Option<Fill> {
        Some(Fill::Color(Color::new(0, 0, 0, self as f32)))
    }
}

impl<T> IntoFill for [T; 1]
where
    T: Into<f64> + Copy,
{
    fn into_fill(self) -> Option<Fill> {
        Some(Fill::Color(Color::new(0, 0, 0, self[0].into() as f32)))
    }
}

impl<T> IntoFill for [T; 3]
where
    T: Into<u8> + Copy,
{
    fn into_fill(self) -> Option<Fill> {
        Some(Fill::Color(Color::rgb(
            self[0].into(),
            self[1].into(),
            self[2].into(),
        )))
    }
}

impl<T> IntoFill for [T; 4]
where
    T: Into<f64> + Copy,
{
    fn into_fill(self) -> Option<Fill> {
        Some(Fill::Color(Color::new(
            self[0].into().clamp(0.0, 255.0) as u8,
            self[1].into().clamp(0.0, 255.0) as u8,
            self[2].into().clamp(0.0, 255.0) as u8,
            self[3].into() as f32,
        )))
    }
}
