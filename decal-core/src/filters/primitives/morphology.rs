use crate::filters::primitives::PrimitiveBuilder;
use crate::filters::{FilterRegion, HasFilterRegion};
use crate::paint::ResourceIri;
use crate::primitives::{FilterInput, PositiveF32Pair};
use crate::utils::{ElementWriter, IsDefault};
use enum_display::EnumDisplay;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum MorphologyOperator {
    #[display("erode")]
    #[default]
    Erode,
    #[display("dilate")]
    Dilate,
}

impl IsDefault for MorphologyOperator {}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
pub struct Morphology {
    input: Option<FilterInput>,
    operator: MorphologyOperator,
    radius: PositiveF32Pair,
    region: FilterRegion,
}

impl Morphology {
    pub(crate) fn new() -> Self {
        Morphology::default()
    }
}

impl ResourceIri for Morphology {}

impl HasFilterRegion for Morphology {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl Display for Morphology {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feMorphology")?
            .write(|out| self.region.fmt(out))?
            .attr("in", self.input.map(|x| (x,)))?
            .attr_if("operator", (self.operator,), !self.operator.is_default())?
            .attr_if("radius", (self.radius,), !self.radius.is_zero())?
            .attr("result", (self.iri(),))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, Morphology> {
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input = Some(input.into());
        self
    }

    pub fn operator(mut self, operator: MorphologyOperator) -> Self {
        self.inner.operator = operator;
        self
    }

    pub fn radius<T>(mut self, radius: T) -> Self
    where
        T: Into<PositiveF32Pair>,
    {
        self.inner.radius = radius.into();
        self
    }
}
