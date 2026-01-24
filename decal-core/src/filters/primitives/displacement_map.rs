use crate::{
    filters::{
        FilterRegion,
        HasFilterRegion,
        primitives::PrimitiveBuilder,
    },
    macros::ff32,
    paint::ResourceIri,
    primitives::{
        ColorInterpolation,
        FilterInput,
    },
    utils::{
        ElementWriter,
        IsDefault,
    },
};
use enum_display::EnumDisplay;
use smart_default::SmartDefault;
use std::fmt::{
    Display,
    Formatter,
};
use strict_num::FiniteF32;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum ChannelSelector {
    #[display("R")]
    R,
    #[display("G")]
    G,
    #[display("B")]
    B,
    #[default]
    #[display("A")]
    A,
}

impl IsDefault for ChannelSelector {}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, SmartDefault)]
pub struct DisplacementMap {
    input: Option<FilterInput>,
    map: Option<FilterInput>,
    scale: FiniteF32,
    x_channel_selector: ChannelSelector,
    y_channel_selector: ChannelSelector,
    region: FilterRegion,
    #[default(ColorInterpolation::LinearRgb)]
    color_interpolation: ColorInterpolation,
}

impl DisplacementMap {
    pub(crate) fn new() -> Self {
        DisplacementMap::default()
    }
}

impl HasFilterRegion for DisplacementMap {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for DisplacementMap {}

impl Display for DisplacementMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feDisplacementMap")?
            .write(|out| self.region.fmt(out))?
            .attrs([
                ("in", self.input.map(|x| (x,))),
                ("in2", self.map.map(|x| (x,))),
            ])?
            .attr_if("scale", self.scale, self.scale.get() != 0.0)?
            .attr_if(
                "xChannelSelector",
                (self.x_channel_selector,),
                !self.x_channel_selector.is_default(),
            )?
            .attr_if(
                "yChannelSelector",
                (self.y_channel_selector,),
                !self.y_channel_selector.is_default(),
            )?
            .attr_if(
                "color-interpolation-filters",
                (&self.color_interpolation,),
                self.color_interpolation != ColorInterpolation::LinearRgb,
            )?
            .attr("result", (self.iri(),))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, DisplacementMap> {
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input = Some(input.into());
        self
    }

    pub fn map<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.map = Some(input.into());
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.inner.scale = ff32!(scale);
        self
    }

    pub fn x_channel(mut self, channel_selector: ChannelSelector) -> Self {
        self.inner.x_channel_selector = channel_selector;
        self
    }

    pub fn y_channel(mut self, channel_selector: ChannelSelector) -> Self {
        self.inner.y_channel_selector = channel_selector;
        self
    }

    pub fn color_interpolation(mut self, value: ColorInterpolation) -> Self {
        self.inner.color_interpolation = value;
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
        ctx.displacement_map()
            .x(0.5)
            .y(0.6)
            .width(110)
            .height(120)
            .finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feDisplacementMap x="0.5" y="0.6" width="110" height="120" result="{}" />"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders() {
        let ctx = FilterContext::default();
        ctx.displacement_map().finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(r#"<feDisplacementMap result="{}" />"#, node.iri()),
        );
    }

    #[test]
    fn renders_with_attrs() {
        let ctx = FilterContext::default();
        let input = FilterInput::source_graphic();
        let map = FilterInput::source_alpha();
        let x_channel = ChannelSelector::R;
        let y_channel = ChannelSelector::B;
        let color_interpolation = ColorInterpolation::SRgb;

        ctx.displacement_map()
            .input(input)
            .map(map)
            .scale(2.5)
            .x_channel(x_channel)
            .y_channel(y_channel)
            .color_interpolation(color_interpolation)
            .finish();

        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feDisplacementMap
    in="{input}"
    in2="{map}"
    scale="2.5"
    xChannelSelector="{x_channel}"
    yChannelSelector="{y_channel}"
    color-interpolation-filters="{color_interpolation}"
    result="{}"
/>
"#,
                node.iri()
            ),
        );
    }
}
