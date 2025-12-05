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
