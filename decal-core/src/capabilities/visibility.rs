use super::Drawable;

/// Capability for controlling the visibility of a node.
pub trait Visibility: Drawable {
    /// Sets whether the node is visible.
    ///
    /// # Note
    /// The node continues to participate in layout computation regardless of
    /// its visibility.
    ///
    /// # Arguments
    /// - `value`: The visibility state of the node.
    ///
    /// # Returns
    /// - [`Self`]
    fn visible(mut self, value: bool) -> Self {
        self.visual_mut().visible = value;
        self
    }
}
