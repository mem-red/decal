use crate::primitives::{Length, Rect};

type Margin = Rect<Length>;

pub trait IntoMargin {
    fn into_margin(self) -> Option<Margin>;
}

impl IntoMargin for Option<Margin> {
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        self
    }
}

impl IntoMargin for Margin {
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(self)
    }
}

impl IntoMargin for Length {
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(Rect {
            top: self,
            right: self,
            bottom: self,
            left: self,
        })
    }
}

impl IntoMargin for [Length; 1] {
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(Rect {
            top: self[0],
            right: self[0],
            bottom: self[0],
            left: self[0],
        })
    }
}

impl IntoMargin for [Length; 2] {
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(Rect {
            top: self[0],
            right: self[1],
            bottom: self[0],
            left: self[1],
        })
    }
}

impl IntoMargin for [Length; 3] {
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(Rect {
            top: self[0],
            right: self[1],
            bottom: self[2],
            left: self[1],
        })
    }
}

impl IntoMargin for [Length; 4] {
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(Rect {
            top: self[0],
            right: self[1],
            bottom: self[2],
            left: self[3],
        })
    }
}

pub trait IntoMarginPair {
    fn into_margin_pair(self) -> Option<(Length, Length)>;
}

impl IntoMarginPair for Length {
    #[inline]
    fn into_margin_pair(self) -> Option<(Length, Length)> {
        Some((self, self))
    }
}

impl IntoMarginPair for [Length; 1] {
    #[inline]
    fn into_margin_pair(self) -> Option<(Length, Length)> {
        Some((self[0], self[0]))
    }
}

impl IntoMarginPair for [Length; 2] {
    #[inline]
    fn into_margin_pair(self) -> Option<(Length, Length)> {
        Some((self[0], self[1]))
    }
}
