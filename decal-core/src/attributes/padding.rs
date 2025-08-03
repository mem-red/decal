use crate::primitives::{Length, Rect};

pub type Padding = Rect<Length>;

impl Padding {
    #[must_use]
    pub const fn new() -> Self {
        Self::from_values(Length::Zero, Length::Zero, Length::Zero, Length::Zero)
    }
}
