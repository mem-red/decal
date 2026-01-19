use crate::{
    filters::{
        FilterRegion,
        HasFilterRegion,
        primitives::PrimitiveBuilder,
    },
    paint::ResourceIri,
    primitives::FilterInput,
    utils::ElementWriter,
};
use std::fmt::{
    Display,
    Formatter,
};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
pub struct Tile {
    input: Option<FilterInput>,
    region: FilterRegion,
}

impl Tile {
    pub(crate) fn new() -> Self {
        Tile::default()
    }
}

impl HasFilterRegion for Tile {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for Tile {}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feTile")?
            .write(|out| self.region.fmt(out))?
            .attr("in", self.input.map(|x| (x,)))?
            .attr("result", (self.iri(),))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, Tile> {
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input = Some(input.into());
        self
    }
}
