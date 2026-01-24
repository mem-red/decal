use crate::{
    filters::{
        FilterRegion,
        HasFilterRegion,
        primitives::PrimitiveBuilder,
    },
    macros::{
        ff32,
        pf32,
    },
    paint::ResourceIri,
    primitives::{
        Color,
        ColorInterpolation,
        FilterInput,
        LightSource,
        PositiveF32Pair,
    },
    utils::ElementWriter,
};
use std::fmt::{
    Display,
    Formatter,
};
use strict_num::{
    FiniteF32,
    PositiveF32,
};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct SpecularLighting {
    input: Option<FilterInput>,
    light_source: LightSource,
    lighting_color: Option<Color>,
    surface_scale: FiniteF32,
    specular_constant: PositiveF32,
    specular_exponent: PositiveF32,
    kernel_unit_length: Option<PositiveF32Pair>,
    region: FilterRegion,
    color_interpolation: ColorInterpolation,
}

impl SpecularLighting {
    pub(crate) fn new(light_source: LightSource) -> Self {
        SpecularLighting {
            input: None,
            light_source,
            lighting_color: None,
            surface_scale: ff32!(1.0),
            specular_constant: pf32!(1.0),
            specular_exponent: pf32!(1.0),
            kernel_unit_length: None,
            region: Default::default(),
            color_interpolation: ColorInterpolation::LinearRgb,
        }
    }
}

impl HasFilterRegion for SpecularLighting {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for SpecularLighting {}

impl Display for SpecularLighting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feSpecularLighting")?
            .write(|out| self.region.fmt(out))?
            .attr("in", self.input.map(|x| (x,)))?
            .attr_if(
                "surfaceScale",
                self.surface_scale,
                self.surface_scale.get() != 1.0,
            )?
            .attr_if(
                "specularConstant",
                self.specular_constant,
                self.specular_constant.get() != 1.0,
            )?
            .attr_if(
                "specularExponent",
                self.specular_exponent,
                self.specular_exponent.get() != 1.0,
            )?
            .attr("kernelUnitLength", self.kernel_unit_length.map(|x| (x,)))?
            .attr("lighting-color", self.lighting_color.map(|x| (x,)))?
            .attr_if(
                "color-interpolation-filters",
                (&self.color_interpolation,),
                self.color_interpolation != ColorInterpolation::LinearRgb,
            )?
            .attr("result", (self.iri(),))?
            .content(|out| self.light_source.fmt(out))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, SpecularLighting> {
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input = Some(input.into());
        self
    }

    pub fn lighting_color<T>(mut self, color: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.inner.lighting_color = color.into();
        self
    }

    pub fn surface_scale(mut self, scale: f32) -> Self {
        self.inner.surface_scale = ff32!(scale);
        self
    }

    pub fn specular_constant(mut self, constant: f32) -> Self {
        self.inner.specular_constant = pf32!(constant);
        self
    }

    pub fn specular_exponent(mut self, exponent: f32) -> Self {
        self.inner.specular_exponent = pf32!(exponent);
        self
    }

    pub fn kernel_unit_length<T, P>(mut self, value: T) -> Self
    where
        T: Into<Option<P>>,
        P: Into<PositiveF32Pair>,
    {
        self.inner.kernel_unit_length = value.into().map(Into::into);
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
        let light_source = LightSource::point_light(1.0, 2.0, 3.0);
        ctx.specular_lighting(light_source)
            .x(0.5)
            .y(0.6)
            .width(110)
            .height(120)
            .finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feSpecularLighting x="0.5" y="0.6" width="110" height="120" result="{}">
    {light_source}
</feSpecularLighting>"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders() {
        let ctx = FilterContext::default();
        let light_source = LightSource::point_light(1.0, 2.0, 3.0);
        ctx.specular_lighting(light_source).finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feSpecularLighting result="{}">{light_source}</feSpecularLighting>"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_with_attrs() {
        let ctx = FilterContext::default();
        let light_source = LightSource::point_light(1.0, 2.0, 3.0);
        let input = FilterInput::source_graphic();
        let color = Color::rgb(10, 15, 20);
        let color_interpolation = ColorInterpolation::SRgb;

        ctx.specular_lighting(light_source)
            .input(input)
            .lighting_color(color)
            .surface_scale(5.0)
            .specular_constant(2.5)
            .specular_exponent(1.5)
            .kernel_unit_length((0.2, 0.3))
            .color_interpolation(color_interpolation)
            .finish();

        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feSpecularLighting
    in="{input}"
    lighting-color="{color}"
    surfaceScale="5"
    specularConstant="2.5"
    specularExponent="1.5"
    kernelUnitLength="0.2 0.3"
    color-interpolation-filters="{color_interpolation}"
    result="{}"
>
    {light_source}
</feSpecularLighting>
"#,
                node.iri()
            ),
        );
    }
}
