use crate::filters::primitives::PrimitiveBuilder;
use crate::filters::{FilterRegion, HasFilterRegion};
use crate::macros::nf32;
use crate::paint::ResourceIri;
use crate::primitives::Color;
use crate::utils::ElementWriter;
use std::fmt::{Display, Formatter};
use strict_num::NormalizedF32;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Flood {
    color: Color,
    opacity: NormalizedF32,
    region: FilterRegion,
}

impl Default for Flood {
    fn default() -> Self {
        Flood {
            color: Color::rgb(0, 0, 0),
            opacity: NormalizedF32::ONE,
            region: Default::default(),
        }
    }
}

impl Flood {
    pub(crate) fn new() -> Self {
        Flood::default()
    }
}

impl ResourceIri for Flood {}

impl HasFilterRegion for Flood {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

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
    pub fn color(mut self, color: Color) -> Self {
        self.inner.color = color;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.inner.opacity = nf32!(opacity);
        self
    }
}
