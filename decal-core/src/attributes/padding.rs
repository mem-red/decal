use crate::primitives::{Length, Rect};

pub type Padding = Rect<Length>;

impl Padding {
    #[must_use]
    pub const fn new() -> Self {
        Self::from_values(Length::Zero, Length::Zero, Length::Zero, Length::Zero)
    }

    pub(crate) fn to_style(&self) -> taffy::Rect<taffy::LengthPercentage> {
        taffy::Rect {
            top: self.top.to_length_percentage(),
            right: self.right.to_length_percentage(),
            bottom: self.bottom.to_length_percentage(),
            left: self.left.to_length_percentage(),
        }
    }
}
