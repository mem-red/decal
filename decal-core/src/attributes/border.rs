use crate::primitives::{Length, Rect};

type BorderWidth = Length<false, true>;

pub trait IntoBorderWidth {
    fn into_border_width(self) -> Option<BorderWidth>;
}

impl IntoBorderWidth for Option<BorderWidth> {
    #[inline]
    fn into_border_width(self) -> Option<BorderWidth> {
        self
    }
}

impl IntoBorderWidth for BorderWidth {
    #[inline]
    fn into_border_width(self) -> Option<BorderWidth> {
        Some(self)
    }
}

type Border = Rect<BorderWidth>;

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

impl IntoBorder for BorderWidth {
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

impl IntoBorder for [BorderWidth; 1] {
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

impl IntoBorder for [BorderWidth; 2] {
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

impl IntoBorder for [BorderWidth; 3] {
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

impl IntoBorder for [BorderWidth; 4] {
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

type BorderPair = (BorderWidth, BorderWidth);

pub trait IntoBorderPair {
    fn into_border_pair(self) -> Option<BorderPair>;
}

impl IntoBorderPair for BorderWidth {
    #[inline]
    fn into_border_pair(self) -> Option<BorderPair> {
        Some((self, self))
    }
}

impl IntoBorderPair for [BorderWidth; 1] {
    #[inline]
    fn into_border_pair(self) -> Option<BorderPair> {
        Some((self[0], self[0]))
    }
}

impl IntoBorderPair for [BorderWidth; 2] {
    #[inline]
    fn into_border_pair(self) -> Option<BorderPair> {
        Some((self[0], self[1]))
    }
}
