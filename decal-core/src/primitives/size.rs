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

    pub fn width(&self) -> T {
        self.width
    }

    pub fn height(&self) -> T {
        self.height
    }
}

impl<const AUTO: bool, const PERCENT: bool> Default for Size<Length<AUTO, PERCENT>> {
    fn default() -> Self {
        Self::from_values(Length::zero(), Length::zero())
    }
}

//

impl<T> From<taffy::geometry::Size<T>> for Size<T>
where
    T: Copy,
{
    fn from(value: taffy::Size<T>) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

impl<const AUTO: bool, const PERCENT: bool> From<Size<Length<AUTO, PERCENT>>>
    for taffy::Size<taffy::Dimension>
{
    fn from(value: Size<Length<AUTO, PERCENT>>) -> Self {
        taffy::Size {
            width: value.width.into(),
            height: value.height.into(),
        }
    }
}

impl<const PERCENT: bool> From<Size<Length<false, PERCENT>>>
    for taffy::Size<taffy::LengthPercentage>
{
    fn from(value: Size<Length<false, PERCENT>>) -> Self {
        taffy::Size {
            width: value.width.into(),
            height: value.height.into(),
        }
    }
}

impl<const AUTO: bool, const PERCENT: bool> From<Size<Length<AUTO, PERCENT>>>
    for taffy::Size<taffy::LengthPercentageAuto>
{
    fn from(value: Size<Length<AUTO, PERCENT>>) -> Self {
        taffy::Size {
            width: value.width.into(),
            height: value.height.into(),
        }
    }
}
