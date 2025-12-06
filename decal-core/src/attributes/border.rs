use crate::primitives::{Length, Rect};

#[derive(Debug, Clone, Copy, Default)]
pub struct Border(pub Rect<Length>);

impl Into<taffy::Rect<taffy::LengthPercentage>> for Border {
    fn into(self) -> taffy::Rect<taffy::LengthPercentage> {
        taffy::Rect {
            top: self.top.into(),
            right: self.right.into(),
            bottom: self.bottom.into(),
            left: self.left.into(),
        }
    }
}

impl std::ops::Deref for Border {
    type Target = Rect<Length>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Border {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait IntoBorder {
    fn into_border(self) -> Option<Border>;
}

impl IntoBorder for Option<Border> {
    fn into_border(self) -> Option<Border> {
        self
    }
}

impl IntoBorder for Border {
    fn into_border(self) -> Option<Border> {
        Some(self)
    }
}

impl IntoBorder for Length {
    fn into_border(self) -> Option<Border> {
        Some(Border(Rect {
            top: self,
            right: self,
            bottom: self,
            left: self,
        }))
    }
}

impl IntoBorder for [Length; 1] {
    fn into_border(self) -> Option<Border> {
        Some(Border(Rect {
            top: self[0],
            right: self[0],
            bottom: self[0],
            left: self[0],
        }))
    }
}

impl IntoBorder for [Length; 2] {
    fn into_border(self) -> Option<Border> {
        Some(Border(Rect {
            top: self[0],
            right: self[1],
            bottom: self[0],
            left: self[1],
        }))
    }
}

impl IntoBorder for [Length; 3] {
    fn into_border(self) -> Option<Border> {
        Some(Border(Rect {
            top: self[0],
            right: self[1],
            bottom: self[2],
            left: self[1],
        }))
    }
}

impl IntoBorder for [Length; 4] {
    fn into_border(self) -> Option<Border> {
        Some(Border(Rect {
            top: self[0],
            right: self[1],
            bottom: self[2],
            left: self[3],
        }))
    }
}

pub trait IntoBorderPair {
    fn into_border_pair(self) -> Option<(Length, Length)>;
}

impl IntoBorderPair for Length {
    fn into_border_pair(self) -> Option<(Length, Length)> {
        Some((self, self))
    }
}

impl IntoBorderPair for [Length; 1] {
    fn into_border_pair(self) -> Option<(Length, Length)> {
        Some((self[0], self[0]))
    }
}

impl IntoBorderPair for [Length; 2] {
    fn into_border_pair(self) -> Option<(Length, Length)> {
        Some((self[0], self[1]))
    }
}
