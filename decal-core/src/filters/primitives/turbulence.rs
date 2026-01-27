use crate::{
    filters::{
        FilterRegion,
        HasFilterRegion,
        primitives::PrimitiveBuilder,
    },
    paint::ResourceIri,
    primitives::{
        ColorInterpolation,
        PositiveF32Pair,
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

/// The type of the turbulence function used by the [`Turbulence`] filter
/// primitive.
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum TurbulenceType {
    /// The standard turbulence function.
    #[default]
    #[display("turbulence")]
    Turbulence,
    /// The fractal noise variant.
    #[display("fractalNoise")]
    FractalNoise,
}

impl IsDefault for TurbulenceType {}

/// The turbulence filter primitive.
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, SmartDefault)]
pub struct Turbulence {
    base_freq: PositiveF32Pair,
    #[default(1)]
    num_octaves: u64,
    seed: u64,
    kind: TurbulenceType,
    region: FilterRegion,
    #[default(ColorInterpolation::LinearRgb)]
    color_interpolation: ColorInterpolation,
}

impl Turbulence {
    /// Creates a new [`Turbulence`] primitive.
    ///
    /// # Returns
    /// - [`Self`]
    pub(crate) fn new() -> Self {
        Turbulence::default()
    }
}

impl HasFilterRegion for Turbulence {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for Turbulence {}

impl Display for Turbulence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feTurbulence")?
            .write(|out| self.region.fmt(out))?
            .attr_if("type", (self.kind,), !self.kind.is_default())?
            .attr_if("seed", (self.seed,), self.seed != 0)?
            .attr_if("baseFrequency", self.base_freq, !self.base_freq.is_zero())?
            .attr_if("numOctaves", (self.num_octaves,), self.num_octaves != 1)?
            .attr_if(
                "color-interpolation-filters",
                (&self.color_interpolation,),
                self.color_interpolation != ColorInterpolation::LinearRgb,
            )?
            .attr("result", (self.iri(),))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, Turbulence> {
    /// Sets the base frequency of the turbulence noise.
    ///
    /// # Arguments
    /// - `base_freq`: The base frequency for the noise function.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn base_freq<T>(mut self, base_freq: T) -> Self
    where
        T: Into<PositiveF32Pair>,
    {
        self.inner.base_freq = base_freq.into();
        self
    }

    /// Sets the number of octaves used to generate the noise.
    ///
    /// # Arguments
    /// - `num_octaves`: The number of noise octaves.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn num_octaves(mut self, num_octaves: u64) -> Self {
        self.inner.num_octaves = num_octaves;
        self
    }

    /// Sets the seed used for noise generation.
    ///
    /// # Arguments
    /// - `seed`: The seed value.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn seed(mut self, seed: u64) -> Self {
        self.inner.seed = seed;
        self
    }

    /// Sets the turbulence function type.
    ///
    /// # Arguments
    /// - `kind`: The [`TurbulenceType`] to use.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn kind(mut self, kind: TurbulenceType) -> Self {
        self.inner.kind = kind;
        self
    }

    /// Configures the turbulence to use fractal noise.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn fractal_noise(mut self) -> Self {
        self.inner.kind = TurbulenceType::FractalNoise;
        self
    }

    /// Sets the color interpolation space used during filtering.
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
        ctx.turbulence()
            .x(0.5)
            .y(0.6)
            .width(110)
            .height(120)
            .finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feTurbulence x="0.5" y="0.6" width="110" height="120" result="{}" />"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders() {
        let ctx = FilterContext::default();
        ctx.turbulence().finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(r#"<feTurbulence result="{}" />"#, node.iri()),
        );
    }

    #[test]
    fn renders_with_attrs() {
        let ctx = FilterContext::default();
        let kind = TurbulenceType::FractalNoise;
        let color_interpolation = ColorInterpolation::SRgb;

        ctx.turbulence()
            .base_freq((2.5, 3.5))
            .num_octaves(64)
            .seed(92)
            .kind(kind)
            .color_interpolation(color_interpolation)
            .finish();

        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feTurbulence
    baseFrequency="2.5 3.5"
    numOctaves="64"
    seed="92"
    type="{kind}"
    color-interpolation-filters="{color_interpolation}"
    result="{}"
/>
"#,
                node.iri()
            ),
        );
    }
}
