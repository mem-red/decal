#[derive(Debug, Copy, Clone, Default)]
pub enum Position {
    #[default]
    Relative,
    Absolute,
}

impl Into<taffy::Position> for Position {
    fn into(self) -> taffy::Position {
        match self {
            Position::Relative => taffy::Position::Relative,
            Position::Absolute => taffy::Position::Absolute,
        }
    }
}
