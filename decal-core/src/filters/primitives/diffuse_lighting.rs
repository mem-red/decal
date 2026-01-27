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

/// The diffuse lighting filter primitive.
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct DiffuseLighting {
    input: Option<FilterInput>,
    light_source: LightSource,
    lighting_color: Option<Color>,
    surface_scale: FiniteF32,
    diffuse_constant: PositiveF32,
    region: FilterRegion,
    color_interpolation: ColorInterpolation,
}

impl DiffuseLighting {
    /// Creates a new [`DiffuseLighting`] primitive with the given light source.
    ///
    /// # Arguments
    /// - `light_source`: The [`LightSource`] used for the lighting.
    ///
    /// # Returns
    /// - [`Self`]
    pub(crate) fn new(light_source: LightSource) -> Self {
        DiffuseLighting {
            input: None,
            light_source,
            lighting_color: None,
            surface_scale: ff32!(1.0),
            diffuse_constant: pf32!(1.0),
            region: Default::default(),
            color_interpolation: ColorInterpolation::LinearRgb,
        }
    }
}

impl HasFilterRegion for DiffuseLighting {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for DiffuseLighting {}

impl Display for DiffuseLighting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feDiffuseLighting")?
            .write(|out| self.region.fmt(out))?
            .attr("in", self.input.map(|x| (x,)))?
            .attr_if(
                "surfaceScale",
                self.surface_scale,
                self.surface_scale.get() != 1.0,
            )?
            .attr_if(
                "diffuseConstant",
                self.diffuse_constant,
                self.diffuse_constant.get() != 1.0,
            )?
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

impl<'a> PrimitiveBuilder<'a, DiffuseLighting> {
    /// Sets the input for the lighting computation.
    ///
    /// # Arguments
    /// - `input`: The [`FilterInput`] used as the source graphic.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input = Some(input.into());
        self
    }

    /// Sets the lighting color applied to the diffuse lighting result.
    ///
    /// # Arguments
    /// - `color`: The [`Color`] used for lighting.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn lighting_color<T>(mut self, color: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.inner.lighting_color = color.into();
        self
    }

    /// Sets the surface scale factor.
    ///
    /// # Arguments
    /// - `scale`: The scale applied to the surface geometry.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn surface_scale(mut self, scale: f32) -> Self {
        self.inner.surface_scale = ff32!(scale);
        self
    }

    /// Sets the diffuse reflection constant.
    ///
    /// # Arguments
    /// - `constant`: The diffuse constant controlling light intensity.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn diffuse_constant(mut self, constant: f32) -> Self {
        self.inner.diffuse_constant = pf32!(constant);
        self
    }

    /// Sets the color interpolation space used during lighting.
    ///
    /// # Arguments
    /// - `value`: The [`ColorInterpolation`] space to apply.
    ///
    /// # Returns
    /// - [`Self`]
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
        ctx.diffuse_lighting(light_source)
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
<feDiffuseLighting x="0.5" y="0.6" width="110" height="120" result="{}">
    {light_source}
</feDiffuseLighting>"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders() {
        let ctx = FilterContext::default();
        let light_source = LightSource::point_light(1.0, 2.0, 3.0);
        ctx.diffuse_lighting(light_source).finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feDiffuseLighting result="{}">{light_source}</feDiffuseLighting>"#,
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

        ctx.diffuse_lighting(light_source)
            .input(input)
            .lighting_color(color)
            .surface_scale(5.0)
            .diffuse_constant(2.5)
            .color_interpolation(color_interpolation)
            .finish();

        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feDiffuseLighting
    in="{input}"
    lighting-color="{color}"
    surfaceScale="5"
    diffuseConstant="2.5"
    color-interpolation-filters="{color_interpolation}"
    result="{}"
>
    {light_source}
</feDiffuseLighting>
"#,
                node.iri()
            ),
        );
    }
}
