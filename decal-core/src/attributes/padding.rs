use crate::primitives::{Length, Rect};

#[derive(Debug, Clone, Copy, Default)]
pub struct Padding(pub Rect<Length>);

impl Into<taffy::Rect<taffy::LengthPercentage>> for Padding {
    fn into(self) -> taffy::Rect<taffy::LengthPercentage> {
        taffy::Rect {
            top: self.top.into(),
            right: self.right.into(),
            bottom: self.bottom.into(),
            left: self.left.into(),
        }
    }
}

impl std::ops::Deref for Padding {
    type Target = Rect<Length>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Padding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait IntoPadding {
    fn into_padding(self) -> Option<Padding>;
}

impl IntoPadding for Option<Padding> {
    fn into_padding(self) -> Option<Padding> {
        self
    }
}

impl IntoPadding for Padding {
    fn into_padding(self) -> Option<Padding> {
        Some(self)
    }
}

impl IntoPadding for Length {
    fn into_padding(self) -> Option<Padding> {
        Some(Padding(Rect {
            top: self,
            right: self,
            bottom: self,
            left: self,
        }))
    }
}

impl IntoPadding for [Length; 1] {
    fn into_padding(self) -> Option<Padding> {
        Some(Padding(Rect {
            top: self[0],
            right: self[0],
            bottom: self[0],
            left: self[0],
        }))
    }
}

impl IntoPadding for [Length; 2] {
    fn into_padding(self) -> Option<Padding> {
        Some(Padding(Rect {
            top: self[0],
            right: self[1],
            bottom: self[0],
            left: self[1],
        }))
    }
}

impl IntoPadding for [Length; 3] {
    fn into_padding(self) -> Option<Padding> {
        Some(Padding(Rect {
            top: self[0],
            right: self[1],
            bottom: self[2],
            left: self[1],
        }))
    }
}

impl IntoPadding for [Length; 4] {
    fn into_padding(self) -> Option<Padding> {
        Some(Padding(Rect {
            top: self[0],
            right: self[1],
            bottom: self[2],
            left: self[3],
        }))
    }
}

pub trait IntoPaddingPair {
    fn into_padding_pair(self) -> Option<(Length, Length)>;
}

impl IntoPaddingPair for Length {
    fn into_padding_pair(self) -> Option<(Length, Length)> {
        Some((self, self))
    }
}

impl IntoPaddingPair for [Length; 1] {
    fn into_padding_pair(self) -> Option<(Length, Length)> {
        Some((self[0], self[0]))
    }
}

impl IntoPaddingPair for [Length; 2] {
    fn into_padding_pair(self) -> Option<(Length, Length)> {
        Some((self[0], self[1]))
    }
}
