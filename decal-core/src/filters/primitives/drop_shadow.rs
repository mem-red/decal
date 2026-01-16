use crate::filters::primitives::PrimitiveBuilder;
use crate::filters::{FilterRegion, HasFilterRegion};
use crate::macros::{ff32, nf32};
use crate::paint::ResourceIri;
use crate::primitives::Color;
use crate::primitives::ColorInterpolation;
use crate::primitives::{FilterInput, PositiveF32Pair};
use crate::utils::ElementWriter;
use smart_default::SmartDefault;
use std::fmt::{Display, Formatter};
use strict_num::{FiniteF32, NormalizedF32};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, SmartDefault)]
pub struct DropShadow {
    input: Option<FilterInput>,
    #[default(ff32!(2.0))]
    dx: FiniteF32,
    #[default(ff32!(2.0))]
    dy: FiniteF32,
    #[default(2.0.into())]
    std_deviation: PositiveF32Pair,
    flood_color: Option<Color>,
    flood_opacity: Option<NormalizedF32>,
    region: FilterRegion,
    #[default(ColorInterpolation::LinearRgb)]
    color_interpolation: ColorInterpolation,
}

impl DropShadow {
    pub(crate) fn new() -> Self {
        DropShadow::default()
    }
}

impl HasFilterRegion for DropShadow {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for DropShadow {}

impl Display for DropShadow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feDropShadow")?
            .write(|out| self.region.fmt(out))?
            .attr("in", self.input.map(|x| (x,)))?
            .attr_if("dx", self.dx, self.dx.get() != 2.0)?
            .attr_if("dy", self.dy, self.dy.get() != 2.0)?
            .attr_if(
                "stdDeviation",
                self.std_deviation,
                self.std_deviation != PositiveF32Pair::from(2.0),
            )?
            .attr("flood-color", self.flood_color.map(|x| (x,)))?
            .attr("flood-opacity", self.flood_opacity)?
            .attr_if(
                "color-interpolation-filters",
                (&self.color_interpolation,),
                self.color_interpolation != ColorInterpolation::LinearRgb,
            )?
            .attr("result", (self.iri(),))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, DropShadow> {
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

    pub fn std_deviation<T>(mut self, std_deviation: T) -> Self
    where
        T: Into<PositiveF32Pair>,
    {
        self.inner.std_deviation = std_deviation.into();
        self
    }

    pub fn flood_color<T>(mut self, color: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.inner.flood_color = color.into();
        self
    }

    pub fn flood_opacity<T>(mut self, opacity: T) -> Self
    where
        T: Into<Option<f32>>,
    {
        self.inner.flood_opacity = opacity.into().map(|x| nf32!(x));
        self
    }

    pub fn color_interpolation(mut self, value: ColorInterpolation) -> Self {
        self.inner.color_interpolation = value;
        self
    }
}
