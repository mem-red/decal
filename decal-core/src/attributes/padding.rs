use crate::primitives::{Length, Rect};

pub type Padding = Rect<Length>;

impl Padding {
    #[must_use]
    pub const fn new() -> Self {
        Self::from_values(
            Length::zero(),
            Length::zero(),
            Length::zero(),
            Length::zero(),
        )
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

pub trait IntoPadding {
    fn into_padding(self) -> Option<Padding>;
}

impl IntoPadding for Option<Padding> {
    fn into_padding(self) -> Option<Padding> {
        self
    }
}

impl IntoPadding for Padding {
    fn into_padding(self) -> Option<Padding> {
        Some(self)
    }
}

impl IntoPadding for (Length, Length) {
    fn into_padding(self) -> Option<Padding> {
        Some(Padding {
            top: self.0,
            right: self.1,
            bottom: self.0,
            left: self.1,
        })
    }
}

impl IntoPadding for (Length, Length, Length, Length) {
    fn into_padding(self) -> Option<Padding> {
        Some(Padding {
            top: self.0,
            right: self.1,
            bottom: self.2,
            left: self.3,
        })
    }
}
