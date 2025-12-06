use crate::primitives::{Length, Rect};

#[derive(Debug, Clone, Copy, Default)]
pub struct Margin(pub Rect<Length>);

impl Into<taffy::Rect<taffy::LengthPercentageAuto>> for Margin {
    fn into(self) -> taffy::Rect<taffy::LengthPercentageAuto> {
        taffy::Rect {
            top: self.top.into(),
            right: self.right.into(),
            bottom: self.bottom.into(),
            left: self.left.into(),
        }
    }
}

impl std::ops::Deref for Margin {
    type Target = Rect<Length>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Margin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait IntoMargin {
    fn into_margin(self) -> Option<Margin>;
}

impl IntoMargin for Option<Margin> {
    fn into_margin(self) -> Option<Margin> {
        self
    }
}

impl IntoMargin for Margin {
    fn into_margin(self) -> Option<Margin> {
        Some(self)
    }
}

impl IntoMargin for Length {
    fn into_margin(self) -> Option<Margin> {
        Some(Margin(Rect {
            top: self,
            right: self,
            bottom: self,
            left: self,
        }))
    }
}

impl IntoMargin for [Length; 1] {
    fn into_margin(self) -> Option<Margin> {
        Some(Margin(Rect {
            top: self[0],
            right: self[0],
            bottom: self[0],
            left: self[0],
        }))
    }
}

impl IntoMargin for [Length; 2] {
    fn into_margin(self) -> Option<Margin> {
        Some(Margin(Rect {
            top: self[0],
            right: self[1],
            bottom: self[0],
            left: self[1],
        }))
    }
}

impl IntoMargin for [Length; 3] {
    fn into_margin(self) -> Option<Margin> {
        Some(Margin(Rect {
            top: self[0],
            right: self[1],
            bottom: self[2],
            left: self[1],
        }))
    }
}

impl IntoMargin for [Length; 4] {
    fn into_margin(self) -> Option<Margin> {
        Some(Margin(Rect {
            top: self[0],
            right: self[1],
            bottom: self[2],
            left: self[3],
        }))
    }
}

pub trait IntoMarginPair {
    fn into_margin_pair(self) -> Option<(Length, Length)>;
}

impl IntoMarginPair for Length {
    fn into_margin_pair(self) -> Option<(Length, Length)> {
        Some((self, self))
    }
}

impl IntoMarginPair for [Length; 1] {
    fn into_margin_pair(self) -> Option<(Length, Length)> {
        Some((self[0], self[0]))
    }
}

impl IntoMarginPair for [Length; 2] {
    fn into_margin_pair(self) -> Option<(Length, Length)> {
        Some((self[0], self[1]))
    }
}
