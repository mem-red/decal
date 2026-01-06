use super::Sealed;
use crate::layout::Node;

pub trait Drawable: Sealed + Sized {
    fn finish(self) -> Node;
}
