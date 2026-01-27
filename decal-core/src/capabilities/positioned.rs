use super::Drawable;
use crate::primitives::Position;

/// Capability for configuring the positioning of a node.
pub trait Positioned: Drawable {
    /// Sets the positioning mode.
    ///
    /// # Arguments
    /// - `value`: The [`Position`] to apply.
    ///
    /// # Returns
    /// - [`Self`]
    fn position(mut self, value: Position) -> Self {
        self.layout_mut().position = value.into();
        self
    }
}
