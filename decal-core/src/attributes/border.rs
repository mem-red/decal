use crate::primitives::{Length, Rect};

#[derive(Debug, Clone, Copy, Default)]
pub struct Padding(pub Rect<Length>);

impl Padding {
    pub(crate) fn to_style(&self) -> taffy::Rect<taffy::LengthPercentage> {
        taffy::Rect {
            top: self.top.to_length_percentage(),
            right: self.right.to_length_percentage(),
            bottom: self.bottom.to_length_percentage(),
            left: self.left.to_length_percentage(),
        }
    }
}

impl std::ops::Deref for Padding {
    type Target = Rect<Length>;
    fn deref(&self) -> &Self::Target {
        &self.0
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
