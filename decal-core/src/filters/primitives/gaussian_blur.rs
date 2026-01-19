use crate::{
    filters::{
        FilterRegion,
        HasFilterRegion,
        primitives::PrimitiveBuilder,
    },
    paint::ResourceIri,
    primitives::{
        ColorInterpolation,
        EdgeMode,
        FilterInput,
        PositiveF32Pair,
    },
    utils::{
        ElementWriter,
        IsDefault,
    },
};
use smart_default::SmartDefault;
use std::fmt::{
    Display,
    Formatter,
};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, SmartDefault)]
pub struct GaussianBlur {
    input: Option<FilterInput>,
    std_deviation: PositiveF32Pair,
    edge_mode: EdgeMode,
    region: FilterRegion,
    #[default(ColorInterpolation::LinearRgb)]
    color_interpolation: ColorInterpolation,
}

impl GaussianBlur {
    pub(crate) fn new() -> Self {
        GaussianBlur::default()
    }
}

impl HasFilterRegion for GaussianBlur {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for GaussianBlur {}

impl Display for GaussianBlur {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feGaussianBlur")?
            .write(|out| self.region.fmt(out))?
            .attr("in", self.input.map(|x| (x,)))?
            .attr_if(
                "stdDeviation",
                self.std_deviation,
                !self.std_deviation.is_zero(),
            )?
            .attr_if("edgeMode", (self.edge_mode,), !self.edge_mode.is_default())?
            .attr_if(
                "color-interpolation-filters",
                (&self.color_interpolation,),
                self.color_interpolation != ColorInterpolation::LinearRgb,
            )?
            .attr("result", (self.iri(),))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, GaussianBlur> {
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input = Some(input.into());
        self
    }

    pub fn std_deviation<T>(mut self, std_deviation: T) -> Self
    where
        T: Into<PositiveF32Pair>,
    {
        self.inner.std_deviation = std_deviation.into();
        self
    }

    pub fn edge_mode(mut self, edge_mode: EdgeMode) -> Self {
        self.inner.edge_mode = edge_mode;
        self
    }

    pub fn color_interpolation(mut self, value: ColorInterpolation) -> Self {
        self.inner.color_interpolation = value;
        self
    }
}
