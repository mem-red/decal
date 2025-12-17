use super::Sealed;
use crate::layout::Node;

pub trait Drawable: Sealed {
    fn build(&self) -> Node;
}
