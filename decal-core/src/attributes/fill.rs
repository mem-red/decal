#[derive(Debug, Clone, Copy)]
pub enum Fill {
    Color,
    Transparent, // ?? Merge this into color ??
    Gradient,
    Pattern,
    Image,
}
