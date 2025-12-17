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

impl Default for Size<Length> {
    fn default() -> Self {
        Self::from_values(Length::zero(), Length::zero())
    }
}

impl Into<taffy::Size<taffy::Dimension>> for Size<Length> {
    fn into(self) -> taffy::Size<taffy::Dimension> {
        taffy::Size {
            width: self.width.into(),
            height: self.height.into(),
        }
    }
}

impl Into<taffy::Size<taffy::LengthPercentage>> for Size<Length> {
    fn into(self) -> taffy::Size<taffy::LengthPercentage> {
        taffy::Size {
            width: self.width.into(),
            height: self.height.into(),
        }
    }
}

impl Into<taffy::Size<taffy::LengthPercentageAuto>> for Size<Length> {
    fn into(self) -> taffy::Size<taffy::LengthPercentageAuto> {
        taffy::Size {
            width: self.width.into(),
            height: self.height.into(),
        }
    }
}
