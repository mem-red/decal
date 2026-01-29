use crate::{
    filters::{
        FilterRegion,
        HasFilterRegion,
        primitives::PrimitiveBuilder,
    },
    macros::nf32,
    paint::ResourceIri,
    primitives::Color,
    utils::ElementWriter,
};
use smart_default::SmartDefault;
use std::fmt::{
    Display,
    Formatter,
};
use strict_num::NormalizedF32;

/// The flood filter primitive.
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, SmartDefault)]
pub struct Flood {
    #[default(Color::rgb(0, 0, 0))]
    color: Color,
    #[default(NormalizedF32::ONE)]
    opacity: NormalizedF32,
    region: FilterRegion,
}

impl Flood {
    /// Creates a new [`Flood`] primitive.
    ///
    /// # Returns
    /// - [`Self`]
    pub(crate) fn new() -> Self {
        Flood::default()
    }
}

impl HasFilterRegion for Flood {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for Flood {}

impl Display for Flood {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feFlood")?
            .write(|out| self.region.fmt(out))?
            .attr("flood-color", (self.color,))?
            .attr_if(
                "flood-opacity",
                self.opacity,
                self.opacity != NormalizedF32::ONE,
            )?
            .attr("result", (self.iri(),))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, Flood> {
    /// Sets the flood fill color.
    ///
    /// # Arguments
    /// - `color`: The [`Color`] value.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn color(mut self, color: Color) -> Self {
        self.inner.color = color;
        self
    }

    /// Sets the opacity of the flood fill.
    ///
    /// # Arguments
    /// - `opacity`: The opacity value where `0.0` is fully transparent and
    ///   `1.0` is fully opaque.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.inner.opacity = nf32!(opacity);
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
        ctx.flood().x(0.5).y(0.6).width(110).height(120).finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feFlood
    x="0.5"
    y="0.6"
    width="110"
    height="120"
    flood-color="{}"
    result="{}"
/>
"#,
                Color::rgb(0, 0, 0),
                node.iri()
            ),
        );
    }

    #[test]
    fn renders() {
        let ctx = FilterContext::default();
        ctx.flood().finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feFlood flood-color="{}" result="{}" />"#,
                Color::rgb(0, 0, 0),
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_with_attrs() {
        let ctx = FilterContext::default();
        let color = Color::rgb(10, 15, 20);
        ctx.flood().color(color).opacity(0.5).finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feFlood
    flood-color="{color}"
    flood-opacity="0.5"
    result="{}"
/>
"#,
                node.iri()
            ),
        );
    }
}
