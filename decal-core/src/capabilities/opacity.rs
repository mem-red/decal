use super::Drawable;

/// Capability for configuring opacity on a node.
pub trait Opacity: Drawable {
    /// Sets the opacity of the node (clamped to `[0.0, 1.0]`).
    ///
    /// # Arguments
    /// - `value`: The opacity value where `0.0` is fully transparent and `1.0`
    ///   is fully opaque.
    ///
    /// # Returns
    /// - [`Self`]
    fn opacity(mut self, value: f32) -> Self {
        self.visual_mut().opacity = value.clamp(0.0, 1.0);
        self
    }
}
