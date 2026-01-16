use crate::filters::primitives::PrimitiveBuilder;
use crate::filters::{FilterRegion, HasFilterRegion};
use crate::paint::ResourceIri;
use crate::primitives::CrossOrigin;
use crate::utils::ElementWriter;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Image {
    href: String,
    cross_origin: Option<CrossOrigin>,
    region: FilterRegion,
}

impl Image {
    pub(crate) fn new(href: &str) -> Self {
        Image {
            href: href.to_string(),
            cross_origin: None,
            region: Default::default(),
        }
    }
}

impl HasFilterRegion for Image {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for Image {}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feImage")?
            .write(|out| self.region.fmt(out))?
            .attr("href", self.href.as_str())?
            .attr("crossorigin", self.cross_origin.map(|x| (x,)))?
            .attr("result", (self.iri(),))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, Image> {
    pub fn cross_origin<T>(mut self, cross_origin: T) -> Self
    where
        T: Into<Option<CrossOrigin>>,
    {
        self.inner.cross_origin = cross_origin.into();
        self
    }
}
