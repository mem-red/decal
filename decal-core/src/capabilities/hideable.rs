use super::Drawable;

/// Capability for toggling the visibility of a node.
pub trait Hideable: Drawable {
    /// Sets whether the node is hidden (display property of the node is set to
    /// [`None`]).
    ///
    /// # Arguments
    /// - `value`: Whether the node should be hidden.
    ///
    /// # Returns
    /// - [`Self`]
    ///
    /// [`None`]: crate::primitives::Display::None
    fn hidden(self, value: bool) -> Self;

    /// Hides the node.
    ///
    /// # Returns
    /// - [`Self`]
    fn hide(self) -> Self {
        self.hidden(true)
    }

    /// Shows the node if it was previously hidden.
    ///
    /// # Returns
    /// - [`Self`]
    fn show(self) -> Self {
        self.hidden(false)
    }
}
