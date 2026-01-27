use super::Sealed;
use crate::layout::Node;

/// Core trait implemented by all node builders.
pub trait Drawable: Sealed + Sized {
    /// Finalizes the builder and produces a concrete [`Node`].
    ///
    /// # Returns
    /// - The concrete [`Node`].
    fn finish(self) -> Node;
}
