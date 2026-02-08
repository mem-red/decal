use super::Drawable;
use crate::filters::Filter;

/// Capability for applying filter effects to a node.
pub trait FilterEffects: Drawable {
    /// Sets a filter effect applied to the node.
    ///
    /// # Arguments
    /// - `value`: The filter configuration convertible into a [`Filter`].
    ///
    /// # Examples
    ///
    /// Applying a single filter to node.
    ///
    /// ```rust
    /// # use decal::prelude::*;
    ///
    /// let scene = decal! {
    ///     Text("hello")
    ///         .fx(Filter::invert(0.5))
    /// };
    /// ```
    ///
    /// Applying multiple filters to node.
    ///
    /// ```rust
    /// # use decal::prelude::*;
    ///
    /// let scene = decal! {
    ///     Text("hello")
    ///         .fx([Filter::blur(5.0), Filter::contrast(1.5)])
    /// };
    /// ```
    ///
    /// Using a custom [`Filter`] builder.
    ///
    /// ```rust
    /// # use decal::prelude::*;
    ///
    /// let scene = decal! {
    ///     Text("hello")
    ///         .fx(Filter::new(|ctx| {
    ///             ctx.gaussian_blur().std_deviation(5.0).finish();
    ///         }))
    /// };
    /// ```
    ///
    /// # Returns
    /// - [`Self`]
    fn fx<T>(mut self, value: T) -> Self
    where
        T: Into<Filter>,
    {
        let filter = value.into();
        self.visual_mut().filter = filter.clone();
        self.add_resources(filter);
        self
    }
}
