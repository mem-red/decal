use crate::primitives::Color;
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
    #[inline]
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
