use super::Drawable;
use crate::primitives::BlendMode;

/// Capability for configuring blending mode on a node.
pub trait Blendable: Drawable {
    /// Sets the blend mode used when compositing the node with underlying
    /// content.
    ///
    /// # Arguments
    /// - `value`: The [`BlendMode`] applied during rendering.
    ///
    /// # Returns
    /// - [`Self`]
    fn blend_mode(mut self, value: BlendMode) -> Self {
        self.visual_mut().blend_mode = value;
        self
    }
}
