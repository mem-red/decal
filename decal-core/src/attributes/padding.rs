use crate::primitives::{Length, Rect};

type PaddingSize = Length<false, true>;

pub trait IntoPaddingSize {
    fn into_padding_size(self) -> Option<PaddingSize>;
}

impl IntoPaddingSize for Option<PaddingSize> {
    #[inline]
    fn into_padding_size(self) -> Option<PaddingSize> {
        self
    }
}

impl IntoPaddingSize for PaddingSize {
    #[inline]
    fn into_padding_size(self) -> Option<PaddingSize> {
        Some(self)
    }
}

type Padding = Rect<PaddingSize>;

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

impl IntoPadding for PaddingSize {
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

impl IntoPadding for [PaddingSize; 1] {
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

impl IntoPadding for [PaddingSize; 2] {
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

impl IntoPadding for [PaddingSize; 3] {
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

impl IntoPadding for [PaddingSize; 4] {
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

type PaddingPair = (PaddingSize, PaddingSize);

pub trait IntoPaddingPair {
    fn into_padding_pair(self) -> Option<PaddingPair>;
}

impl IntoPaddingPair for PaddingSize {
    #[inline]
    fn into_padding_pair(self) -> Option<PaddingPair> {
        Some((self, self))
    }
}

impl IntoPaddingPair for [PaddingSize; 1] {
    #[inline]
    fn into_padding_pair(self) -> Option<PaddingPair> {
        Some((self[0], self[0]))
    }
}

impl IntoPaddingPair for [PaddingSize; 2] {
    #[inline]
    fn into_padding_pair(self) -> Option<PaddingPair> {
        Some((self[0], self[1]))
    }
}
