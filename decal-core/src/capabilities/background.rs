use super::Drawable;
use crate::attributes::IntoPaintStack;

/// Capability for configuring background paint on a node.
pub trait Background: Drawable {
    /// Sets the background paint for the node.
    ///
    /// # Arguments
    /// - `value`: The background paint configuration convertible into a paint
    ///   stack using [`IntoPaintStack`].
    ///
    /// # Returns
    /// - [`Self`]
    fn background<T>(mut self, value: T) -> Self
    where
        T: IntoPaintStack,
    {
        let background = value.into_paint_stack();
        self.visual_mut().background = background.clone();
        self.add_resources(background);
        self
    }

    /// Shorthand alias for [`background`].
    ///
    /// # Arguments
    /// - `value`: The background paint configuration convertible into a paint
    ///   stack using [`IntoPaintStack`].
    ///
    /// # Returns
    /// - [`Self`]
    ///
    /// [`background`]: Background::background
    fn bg<T>(self, value: T) -> Self
    where
        T: IntoPaintStack,
    {
        self.background(value)
    }
}
