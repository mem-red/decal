use crate::primitives::{Length, Rect};

type Border = Rect<Length>;

pub trait IntoBorder {
    fn into_border(self) -> Option<Border>;
}

impl IntoBorder for Option<Border> {
    #[inline]
    fn into_border(self) -> Option<Border> {
        self
    }
}

impl IntoBorder for Border {
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(self)
    }
}

impl IntoBorder for Length {
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(Rect {
            top: self,
            right: self,
            bottom: self,
            left: self,
        })
    }
}

impl IntoBorder for [Length; 1] {
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(Rect {
            top: self[0],
            right: self[0],
            bottom: self[0],
            left: self[0],
        })
    }
}

impl IntoBorder for [Length; 2] {
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(Rect {
            top: self[0],
            right: self[1],
            bottom: self[0],
            left: self[1],
        })
    }
}

impl IntoBorder for [Length; 3] {
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(Rect {
            top: self[0],
            right: self[1],
            bottom: self[2],
            left: self[1],
        })
    }
}

impl IntoBorder for [Length; 4] {
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(Rect {
            top: self[0],
            right: self[1],
            bottom: self[2],
            left: self[3],
        })
    }
}

pub trait IntoBorderPair {
    fn into_border_pair(self) -> Option<(Length, Length)>;
}

impl IntoBorderPair for Length {
    #[inline]
    fn into_border_pair(self) -> Option<(Length, Length)> {
        Some((self, self))
    }
}

impl IntoBorderPair for [Length; 1] {
    #[inline]
    fn into_border_pair(self) -> Option<(Length, Length)> {
        Some((self[0], self[0]))
    }
}

impl IntoBorderPair for [Length; 2] {
    #[inline]
    fn into_border_pair(self) -> Option<(Length, Length)> {
        Some((self[0], self[1]))
    }
}
