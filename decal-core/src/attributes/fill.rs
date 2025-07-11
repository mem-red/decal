#[derive(Debug)]
pub enum Fill {
    Color,
    Transparent, // ?? Merge this into color ??
    Gradient,
    Pattern,
    Image,
}
