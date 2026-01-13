use crate::filters::primitives::PrimitiveBuilder;
use crate::filters::{FilterRegion, HasFilterRegion};
use crate::macros::ff32;
use crate::paint::ResourceIri;
use crate::primitives::{FilterInput, PositiveF32Pair};
use crate::utils::ElementWriter;
use std::fmt::{Display, Formatter};
use strict_num::FiniteF32;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct DropShadow {
    input: Option<FilterInput>,
    dx: FiniteF32,
    dy: FiniteF32,
    std_deviation: PositiveF32Pair,
    region: FilterRegion,
}

impl Default for DropShadow {
    fn default() -> Self {
        Self {
            input: None,
            dx: ff32!(2.0),
            dy: ff32!(2.0),
            std_deviation: 2.0.into(),
            region: Default::default(),
        }
    }
}

impl DropShadow {
    pub(crate) fn new() -> Self {
        DropShadow::default()
    }
}

impl ResourceIri for DropShadow {}

impl HasFilterRegion for DropShadow {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

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
}
