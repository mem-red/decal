/// Controls how a node and its children participate in layout.
#[derive(Debug, Clone, Copy, Default)]
pub enum Display {
    /// Uses the block layout algorithm.
    #[default]
    Block,
    /// Uses the flexbox layout algorithm.
    Flex,
    /// Uses the CSS Grid layout algorithm.
    Grid,
    /// Hides the node.
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
