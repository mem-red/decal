#[derive(Debug, Copy, Clone)]
pub struct Rect<T>
where
    T: Copy,
{
    pub(crate) top: T,
    pub(crate) right: T,
    pub(crate) bottom: T,
    pub(crate) left: T,
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

    pub fn top(&mut self, value: T) -> &mut Self {
        self.top = value;
        self
    }

    pub fn right(&mut self, value: T) -> &mut Self {
        self.right = value;
        self
    }

    pub fn bottom(&mut self, value: T) -> &mut Self {
        self.bottom = value;
        self
    }

    pub fn left(&mut self, value: T) -> &mut Self {
        self.left = value;
        self
    }

    pub fn horizontal(&mut self, value: T) -> &mut Self {
        self.right = value;
        self.left = value;
        self
    }

    pub fn vertical(&mut self, value: T) -> &mut Self {
        self.top = value;
        self.bottom = value;
        self
    }
}
