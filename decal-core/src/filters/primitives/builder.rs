use crate::filters::{
    FilterRegion,
    FilterRegionConfig,
    HasFilterRegion,
    context::{
        FilterContext,
        PrimitiveNode,
    },
    primitives::FilterPrimitive,
};

/// A fluent builder for constructing and configuring a single filter primitive.
///
/// A primitive is only added to the filter graph when [`finish`] is called.
///
/// [`finish`]: Self::finish
#[derive(Debug)]
pub struct PrimitiveBuilder<'a, T>
where
    T: Into<FilterPrimitive> + HasFilterRegion,
{
    pub(crate) ctx: &'a FilterContext,
    pub(crate) inner: T,
}

impl<'a, T> PrimitiveBuilder<'a, T>
where
    T: Into<FilterPrimitive> + HasFilterRegion,
{
    /// Creates a new [`PrimitiveBuilder`] wrapping the given primitive.
    ///
    /// # Arguments
    /// - `ctx`: The [`FilterContext`] that owns the filter graph.
    /// - `inner`: The concrete filter primitive being configured.
    ///
    /// # Returns
    /// - `Self`
    pub(crate) fn new(ctx: &'a FilterContext, inner: T) -> Self {
        Self { ctx, inner }
    }

    /// Finalizes the primitive and registers it with the filter context.
    ///
    /// # Returns
    /// - [`PrimitiveNode`] referencing the finalized filter primitive. This can
    ///   be used as an input to filter primitives.
    pub fn finish(self) -> PrimitiveNode {
        self.ctx.get_or_add_primitive(self.inner.into())
    }
}

impl<'a, T> HasFilterRegion for PrimitiveBuilder<'a, T>
where
    T: Into<FilterPrimitive> + HasFilterRegion,
{
    fn region_mut(&mut self) -> &mut FilterRegion {
        self.inner.region_mut()
    }
}

impl<'a, T> FilterRegionConfig for PrimitiveBuilder<'a, T> where
    T: Into<FilterPrimitive> + HasFilterRegion
{
}
