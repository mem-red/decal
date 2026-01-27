use super::Drawable;
use crate::primitives::{
    AlignContent,
    AlignItems,
    JustifyContent,
};

/// Capability for configuring alignment behavior of child elements within a
/// container.
pub trait ContainerAlignment: Drawable {
    /// Sets how child items are aligned along the cross axis.
    ///
    /// # Arguments
    /// - `value`: The [`AlignItems`] behavior.
    ///
    /// # Returns
    /// - [`Self`]
    fn align_items<T>(mut self, value: T) -> Self
    where
        T: Into<Option<AlignItems>>,
    {
        self.layout_mut().align_items = value.into().map(Into::into);
        self
    }

    /// Sets how lines of content are aligned within the container.
    ///
    /// # Arguments
    /// - `value`: The [`AlignContent`] behavior.
    ///
    /// # Returns
    /// - [`Self`]
    fn align_content<T>(mut self, value: T) -> Self
    where
        T: Into<Option<AlignContent>>,
    {
        self.layout_mut().align_content = value.into().map(Into::into);
        self
    }

    /// Sets how child items are distributed along the main axis.
    ///
    /// # Arguments
    /// - `value`: The [`JustifyContent`] behavior.
    ///
    /// # Returns
    /// - [`Self`]
    fn justify_content<T>(mut self, value: T) -> Self
    where
        T: Into<Option<JustifyContent>>,
    {
        self.layout_mut().justify_content = value.into().map(Into::into);
        self
    }
}
