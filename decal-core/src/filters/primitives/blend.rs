use crate::filters::primitives::PrimitiveBuilder;
use crate::filters::{FilterRegion, HasFilterRegion};
use crate::paint::ResourceIri;
use crate::primitives::{BlendMode, FilterInput};
use crate::utils::{ElementWriter, IsDefault};
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
pub struct Blend {
    input: Option<FilterInput>,
    input2: Option<FilterInput>,
    mode: BlendMode,
    region: FilterRegion,
}

impl Blend {
    pub(crate) fn new() -> Self {
        Blend::default()
    }
}

impl HasFilterRegion for Blend {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for Blend {}

impl Display for Blend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feBlend")?
            .write(|out| self.region.fmt(out))?
            .attrs([
                ("in", self.input.map(|x| (x,))),
                ("in2", self.input2.map(|x| (x,))),
            ])?
            .attr_if("mode", (self.mode,), !self.mode.is_default())?
            .attr("result", (self.iri(),))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, Blend> {
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input = Some(input.into());
        self
    }

    pub fn input2<T>(mut self, input2: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input2 = Some(input2.into());
        self
    }

    pub fn mode(mut self, mode: BlendMode) -> Self {
        self.inner.mode = mode;
        self
    }
}
