use crate::prelude::{Overflow, Point};

#[derive(Debug, Clone, Copy, Default)]
pub struct OverflowWrapper(pub Point<Overflow>);

impl Into<taffy::Point<taffy::Overflow>> for OverflowWrapper {
    fn into(self) -> taffy::Point<taffy::Overflow> {
        taffy::Point {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}

impl std::ops::Deref for OverflowWrapper {
    type Target = Point<Overflow>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OverflowWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait IntoOverflow {
    fn into_overflow(self) -> Option<OverflowWrapper>;
}

impl IntoOverflow for Option<OverflowWrapper> {
    fn into_overflow(self) -> Option<OverflowWrapper> {
        self
    }
}

impl IntoOverflow for OverflowWrapper {
    fn into_overflow(self) -> Option<OverflowWrapper> {
        Some(self)
    }
}

impl IntoOverflow for Overflow {
    fn into_overflow(self) -> Option<OverflowWrapper> {
        Some(OverflowWrapper(Point { x: self, y: self }))
    }
}

impl IntoOverflow for [Overflow; 1] {
    fn into_overflow(self) -> Option<OverflowWrapper> {
        Some(OverflowWrapper(Point {
            x: self[0],
            y: self[0],
        }))
    }
}

impl IntoOverflow for [Overflow; 2] {
    fn into_overflow(self) -> Option<OverflowWrapper> {
        Some(OverflowWrapper(Point {
            x: self[0],
            y: self[1],
        }))
    }
}
