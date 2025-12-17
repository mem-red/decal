#[derive(Debug, Clone, Copy, Default)]
pub enum Overflow {
    #[default]
    Visible,
    Hidden,
}

impl Into<taffy::Overflow> for Overflow {
    fn into(self) -> taffy::Overflow {
        match self {
            Overflow::Visible => taffy::Overflow::Visible,
            Overflow::Hidden => taffy::Overflow::Hidden,
        }
    }
}
