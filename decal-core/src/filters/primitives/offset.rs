use crate::{
    filters::{
        FilterRegion,
        HasFilterRegion,
        primitives::PrimitiveBuilder,
    },
    macros::ff32,
    paint::ResourceIri,
    primitives::FilterInput,
    utils::ElementWriter,
};
use std::fmt::{
    Display,
    Formatter,
};
use strict_num::FiniteF32;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
pub struct Offset {
    input: Option<FilterInput>,
    dx: FiniteF32,
    dy: FiniteF32,
    region: FilterRegion,
}

impl Offset {
    pub(crate) fn new() -> Self {
        Offset::default()
    }
}

impl HasFilterRegion for Offset {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for Offset {}

impl Display for Offset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feOffset")?
            .write(|out| self.region.fmt(out))?
            .attr("in", self.input.map(|x| (x,)))?
            .attr_if("dx", self.dx, self.dx.get() != 0.0)?
            .attr_if("dy", self.dy, self.dy.get() != 0.0)?
            .attr("result", (self.iri(),))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, Offset> {
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input = Some(input.into());
        self
    }

    pub fn dx(mut self, dx: f32) -> Self {
        self.inner.dx = ff32!(dx);
        self
    }

    pub fn dy(mut self, dy: f32) -> Self {
        self.inner.dy = ff32!(dy);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        filters::{
            FilterContext,
            FilterRegionConfig,
        },
        test_utils::assert_xml,
    };

    #[test]
    fn renders_with_filter_region() {
        let ctx = FilterContext::default();
        ctx.offset().x(0.5).y(0.6).width(110).height(120).finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feOffset x="0.5" y="0.6" width="110" height="120" result="{}" />"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders() {
        let ctx = FilterContext::default();
        ctx.offset().finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(r#"<feOffset result="{}" />"#, node.iri()),
        );
    }

    #[test]
    fn renders_with_attrs() {
        let ctx = FilterContext::default();
        let input = FilterInput::source_graphic();
        ctx.offset().input(input).dx(2.5).dy(4.5).finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feOffset in="{input}" dx="2.5" dy="4.5" result="{}" />"#,
                node.iri()
            ),
        );
    }
}
