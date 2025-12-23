use crate::primitives::Length;

#[derive(Debug, Copy, Clone)]
pub struct Corner<T>
where
    T: Copy,
{
    pub top_left: T,
    pub top_right: T,
    pub bottom_right: T,
    pub bottom_left: T,
}

impl<T> Corner<T>
where
    T: Copy,
{
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
