#[derive(Debug, Copy, Clone, Default)]
pub struct Point<T>
where
    T: Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Copy,
{
    #[must_use]
    pub const fn from_values(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Into<taffy::Point<T>> for Point<T>
where
    T: Copy,
{
    fn into(self) -> taffy::Point<T> {
        taffy::Point {
            x: self.x,
            y: self.y,
        }
    }
}
