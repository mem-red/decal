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

impl Default for Rect<Length> {
    fn default() -> Self {
        Self::from_values(
            Length::zero(),
            Length::zero(),
            Length::zero(),
            Length::zero(),
        )
    }
}
