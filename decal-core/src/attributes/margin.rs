use crate::primitives::{Length, Rect};

#[derive(Debug, Clone, Copy, Default)]
pub struct Margin(pub Rect<Length>);

impl Margin {
    pub(crate) fn to_style(&self) -> taffy::Rect<taffy::LengthPercentageAuto> {
        taffy::Rect {
            top: self.top.to_length_percentage_auto(),
            right: self.right.to_length_percentage_auto(),
            bottom: self.bottom.to_length_percentage_auto(),
            left: self.left.to_length_percentage_auto(),
        }
    }
}

impl std::ops::Deref for Margin {
    type Target = Rect<Length>;
    fn deref(&self) -> &Self::Target {
        &self.0
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
