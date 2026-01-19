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
    pub(crate) fn new(ctx: &'a FilterContext, inner: T) -> Self {
        Self { ctx, inner }
    }

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
