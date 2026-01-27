use super::Drawable;
use crate::primitives::Transform;

/// Capability for configuring geometric transformations on a node.
pub trait Transformation: Drawable {
    /// Sets the transformation applied to the node.
    ///
    /// # Arguments
    /// - `value`: The [`Transform`] to apply.
    ///
    /// # Returns
    /// - [`Self`]
    fn transform<T>(mut self, value: T) -> Self
    where
        T: Into<Option<Transform>>,
    {
        self.visual_mut().transform = value.into().unwrap_or_default();
        self
    }
}
