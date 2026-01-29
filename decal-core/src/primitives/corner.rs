use crate::primitives::Length;

/// The four-corner value typically used to describe per-corner properties.
#[derive(Debug, Copy, Clone)]
pub struct Corner<T>
where
    T: Copy,
{
    /// The top-left corner value.
    pub top_left: T,
    /// The top-right corner value.
    pub top_right: T,
    /// The bottom-right corner value.
    pub bottom_right: T,
    /// The bottom-left corner value.
    pub bottom_left: T,
}

impl<T> Corner<T>
where
    T: Copy,
{
    /// Creates a new [`Corner`] instance.
    ///
    /// # Arguments
    /// - `top_left`: The top-left corner value.
    /// - `top_right`: The top-right corner value.
    /// - `bottom_right`: The bottom-right corner value.
    /// - `bottom_left`: The bottom-left corner value.
    ///
    /// # Returns
    /// - [`Self`]
    #[must_use]
    pub const fn from_values(top_left: T, top_right: T, bottom_right: T, bottom_left: T) -> Self {
        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }
}

impl<const AUTO: bool, const PERCENT: bool> Default for Corner<Length<AUTO, PERCENT>> {
    fn default() -> Self {
        Self::from_values(
            Length::zero(),
            Length::zero(),
            Length::zero(),
            Length::zero(),
        )
    }
}
