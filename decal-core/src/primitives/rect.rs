use crate::primitives::Length;

#[derive(Debug, Copy, Clone)]
pub struct Rect<T>
where
    T: Copy,
{
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl<T> Rect<T>
where
    T: Copy,
{
    #[must_use]
    pub const fn from_values(top: T, right: T, bottom: T, left: T) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

impl<const AUTO: bool, const PERCENT: bool> Default for Rect<Length<AUTO, PERCENT>> {
    fn default() -> Self {
        Self::from_values(
            Length::zero(),
            Length::zero(),
            Length::zero(),
            Length::zero(),
        )
    }
}

impl<const PERCENT: bool> Into<taffy::Rect<taffy::LengthPercentage>>
    for Rect<Length<false, PERCENT>>
{
    fn into(self) -> taffy::Rect<taffy::LengthPercentage> {
        taffy::Rect {
            top: self.top.into(),
            right: self.right.into(),
            bottom: self.bottom.into(),
            left: self.left.into(),
        }
    }
}

impl<const AUTO: bool, const PERCENT: bool> Into<taffy::Rect<taffy::LengthPercentageAuto>>
    for Rect<Length<AUTO, PERCENT>>
{
    fn into(self) -> taffy::Rect<taffy::LengthPercentageAuto> {
        taffy::Rect {
            top: self.top.into(),
            right: self.right.into(),
            bottom: self.bottom.into(),
            left: self.left.into(),
        }
    }
}
