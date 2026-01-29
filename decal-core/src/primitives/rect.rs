use crate::primitives::Length;

/// The rectangle defined by values for each edge.
#[derive(Debug, Copy, Clone)]
pub struct Rect<T>
where
    T: Copy,
{
    /// The top edge value.
    pub top: T,
    /// The right edge value.
    pub right: T,
    /// The bottom edge value.
    pub bottom: T,
    /// The left edge value.
    pub left: T,
}

impl<T> Rect<T>
where
    T: Copy,
{
    /// Creates a new [`Rect`] instance.
    ///
    /// # Arguments
    /// - `top`: The top edge value.
    /// - `right`: The right edge value.
    /// - `bottom`: The bottom edge value.
    /// - `left`: The left edge value.
    ///
    /// # Returns
    /// - [`Self`]
    #[must_use]
    pub const fn from_values(top: T, right: T, bottom: T, left: T) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Returns the rectangle edges as a tuple in `top`, `right`, `bottom`,
    /// `left` order.
    #[must_use]
    pub(crate) const fn tuple(&self) -> (T, T, T, T) {
        (self.top, self.right, self.bottom, self.left)
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

impl From<taffy::Rect<f32>> for Rect<f32> {
    fn from(value: taffy::Rect<f32>) -> Self {
        Self {
            top: value.top,
            right: value.right,
            bottom: value.bottom,
            left: value.left,
        }
    }
}
