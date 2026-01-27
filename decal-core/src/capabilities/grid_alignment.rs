use super::Drawable;
use crate::primitives::JustifyItems;

/// Capability for configuring item alignment behavior within a grid container.
pub trait GridAlignment: Drawable {
    /// Sets how grid items are aligned along the inline axis within their grid
    /// areas.
    ///
    /// # Arguments
    /// - `value`: The [`JustifyItems`] behavior.
    ///
    /// # Returns
    /// - [`Self`]
    fn justify_items<T>(mut self, value: T) -> Self
    where
        T: Into<Option<JustifyItems>>,
    {
        self.layout_mut().justify_items = value.into().map(Into::into);
        self
    }
}
