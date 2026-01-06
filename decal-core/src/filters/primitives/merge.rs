use crate::filters::primitives::PrimitiveBuilder;
use crate::filters::{FilterRegion, HasFilterRegion};
use crate::paint::ResourceIri;
use crate::primitives::FilterInput;
use crate::utils::ElementWriter;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub struct Merge {
    inputs: Vec<FilterInput>,
    region: FilterRegion,
}

impl Merge {
    pub(crate) fn new() -> Self {
        Merge::default()
    }
}

impl HasFilterRegion for Merge {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for Merge {}

impl Display for Merge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.inputs.is_empty() {
            return Ok(());
        }

        ElementWriter::new(f, "feMerge")?
            .write(|out| self.region.fmt(out))?
            .attr("result", (self.iri(),))?
            .content(|out| {
                self.inputs.iter().try_for_each(|node| {
                    ElementWriter::new(out, "feMergeNode")?
                        .attr("in", (node,))?
                        .close()
                })
            })?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, Merge> {
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.inputs.push(input.into());
        self
    }

    pub fn inputs<I, T>(mut self, inputs: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<FilterInput>,
    {
        self.inner.inputs.extend(inputs.into_iter().map(Into::into));
        self
    }
}
