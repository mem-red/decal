use crate::attributes::Color;

#[derive(Debug, Clone, Copy, Default)]
pub enum Fill {
    #[default]
    None,
    Color(Color),
    Gradient,
    Pattern,
    Image,
}
