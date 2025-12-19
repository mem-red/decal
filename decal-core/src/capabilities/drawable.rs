use super::Sealed;
use crate::layout::Node;

pub trait Drawable: Sealed + Sized {
    fn build(self) -> Node;
}
