use crate::filters::context::{FilterContext, PrimitiveNode};
use crate::filters::primitives::FilterPrimitive;

#[derive(Debug)]
pub struct PrimitiveBuilder<'a, T>
where
    T: Into<FilterPrimitive>,
{
    pub(crate) ctx: &'a FilterContext,
    pub(crate) inner: T,
}

impl<'a, T> PrimitiveBuilder<'a, T>
where
    T: Into<FilterPrimitive>,
{
    pub(crate) fn new(ctx: &'a FilterContext, inner: T) -> Self {
        Self { ctx, inner }
    }

    pub fn finish(self) -> PrimitiveNode {
        self.ctx.get_or_add_primitive(self.inner.into())
    }
}
