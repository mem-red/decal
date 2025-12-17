use crate::primitives::{Length, Rect};

type Padding = Rect<Length>;

pub trait IntoPadding {
    fn into_padding(self) -> Option<Padding>;
}

impl IntoPadding for Option<Padding> {
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        self
    }
}

impl IntoPadding for Padding {
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(self)
    }
}

impl IntoPadding for Length {
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(Rect {
            top: self,
            right: self,
            bottom: self,
            left: self,
        })
    }
}

impl IntoPadding for [Length; 1] {
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(Rect {
            top: self[0],
            right: self[0],
            bottom: self[0],
            left: self[0],
        })
    }
}

impl IntoPadding for [Length; 2] {
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(Rect {
            top: self[0],
            right: self[1],
            bottom: self[0],
            left: self[1],
        })
    }
}

impl IntoPadding for [Length; 3] {
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(Rect {
            top: self[0],
            right: self[1],
            bottom: self[2],
            left: self[1],
        })
    }
}

impl IntoPadding for [Length; 4] {
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(Rect {
            top: self[0],
            right: self[1],
            bottom: self[2],
            left: self[3],
        })
    }
}

pub trait IntoPaddingPair {
    fn into_padding_pair(self) -> Option<(Length, Length)>;
}

impl IntoPaddingPair for Length {
    #[inline]
    fn into_padding_pair(self) -> Option<(Length, Length)> {
        Some((self, self))
    }
}

impl IntoPaddingPair for [Length; 1] {
    #[inline]
    fn into_padding_pair(self) -> Option<(Length, Length)> {
        Some((self[0], self[0]))
    }
}

impl IntoPaddingPair for [Length; 2] {
    #[inline]
    fn into_padding_pair(self) -> Option<(Length, Length)> {
        Some((self[0], self[1]))
    }
}
