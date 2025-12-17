#[derive(Debug, Clone, Copy, Default)]
pub enum Display {
    #[default]
    Block,
    Flex,
    Grid,
    None,
}

impl Into<taffy::Display> for Display {
    fn into(self) -> taffy::Display {
        match self {
            Display::Block => taffy::Display::Block,
            Display::Flex => taffy::Display::Flex,
            Display::Grid => taffy::Display::Grid,
            Display::None => taffy::Display::None,
        }
    }
}
