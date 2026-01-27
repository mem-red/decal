use super::Drawable;
use crate::attributes::IntoGap;

/// Capability for configuring gaps between child elements of a container.
pub trait Gap: Drawable {
    /// Sets the gap between child elements along both axes.
    ///
    /// # Arguments
    /// - `value`: The gap definition convertible using [`IntoGap`].
    ///
    /// # Returns
    /// - [`Self`]
    fn gap<T>(mut self, value: T) -> Self
    where
        T: IntoGap,
    {
        self.layout_mut().gap = value.into_gap().unwrap_or_default().into();
        self
    }
}
