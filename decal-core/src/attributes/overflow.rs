use crate::primitives::{
    Overflow,
    Point,
};

type OverflowXY = Point<Overflow>;

impl Into<taffy::Point<taffy::Overflow>> for OverflowXY {
    #[inline]
    fn into(self) -> taffy::Point<taffy::Overflow> {
        taffy::Point {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}

pub trait IntoOverflow {
    fn into_overflow(self) -> Option<OverflowXY>;
}

impl IntoOverflow for Option<OverflowXY> {
    #[inline]
    fn into_overflow(self) -> Option<OverflowXY> {
        self
    }
}

impl IntoOverflow for OverflowXY {
    #[inline]
    fn into_overflow(self) -> Option<OverflowXY> {
        Some(self)
    }
}

impl IntoOverflow for Overflow {
    #[inline]
    fn into_overflow(self) -> Option<OverflowXY> {
        Some(Point { x: self, y: self })
    }
}

impl IntoOverflow for [Overflow; 1] {
    #[inline]
    fn into_overflow(self) -> Option<OverflowXY> {
        Some(Point {
            x: self[0],
            y: self[0],
        })
    }
}

impl IntoOverflow for [Overflow; 2] {
    #[inline]
    fn into_overflow(self) -> Option<OverflowXY> {
        Some(Point {
            x: self[0],
            y: self[1],
        })
    }
}
