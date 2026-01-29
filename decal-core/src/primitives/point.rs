/// The two-dimensional point defined by `x` and `y` coordinates.
#[derive(Debug, Copy, Clone, Default)]
pub struct Point<T>
where
    T: Copy,
{
    /// The horizontal coordinate.
    pub x: T,
    /// The vertical coordinate.
    pub y: T,
}

impl<T> Point<T>
where
    T: Copy,
{
    /// Creates a new [`Point`] instance.
    ///
    /// # Arguments
    /// - `x`: The horizontal coordinate.
    /// - `y`: The vertical coordinate.
    ///
    /// # Returns
    /// - [`Self`]
    #[must_use]
    pub const fn from_values(x: T, y: T) -> Self {
        Self { x, y }
    }
}
