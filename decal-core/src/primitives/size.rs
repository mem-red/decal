use crate::primitives::Length;

#[derive(Debug, Copy, Clone)]
pub struct Size<T>
where
    T: Copy,
{
    pub width: T,
    pub height: T,
}

impl<T> Size<T>
where
    T: Copy,
{
    #[must_use]
    pub const fn from_values(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl<const AUTO: bool, const PERCENT: bool> Default for Size<Length<AUTO, PERCENT>> {
    fn default() -> Self {
        Self::from_values(Length::zero(), Length::zero())
    }
}

impl<const AUTO: bool, const PERCENT: bool> Into<taffy::Size<taffy::Dimension>>
    for Size<Length<AUTO, PERCENT>>
{
    fn into(self) -> taffy::Size<taffy::Dimension> {
        taffy::Size {
            width: self.width.into(),
            height: self.height.into(),
        }
    }
}

impl<const PERCENT: bool> Into<taffy::Size<taffy::LengthPercentage>>
    for Size<Length<false, PERCENT>>
{
    fn into(self) -> taffy::Size<taffy::LengthPercentage> {
        taffy::Size {
            width: self.width.into(),
            height: self.height.into(),
        }
    }
}

impl<const AUTO: bool, const PERCENT: bool> Into<taffy::Size<taffy::LengthPercentageAuto>>
    for Size<Length<AUTO, PERCENT>>
{
    fn into(self) -> taffy::Size<taffy::LengthPercentageAuto> {
        taffy::Size {
            width: self.width.into(),
            height: self.height.into(),
        }
    }
}
