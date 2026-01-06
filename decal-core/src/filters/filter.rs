use crate::filters::context::FilterContext;
use crate::filters::primitives::FilterPrimitive;
use crate::filters::{FilterRegion, FilterRegionConfig, HasFilterRegion};
use crate::paint::ResourceIri;
use crate::primitives::{FilterUnits, PrimitiveUnits};
use crate::utils::{ElementWriter, IsDefault};
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub struct Filter {
    filter_units: FilterUnits,
    primitive_units: PrimitiveUnits,
    primitives: Vec<FilterPrimitive>,
    region: FilterRegion,
}

impl Filter {
    pub fn new<T>(build: T) -> Self
    where
        T: FnOnce(&mut FilterContext),
    {
        Filter {
            primitives: {
                let mut ctx = FilterContext::default();
                build(&mut ctx);
                ctx.into_primitives()
            },
            ..Default::default()
        }
    }

    pub fn filter_units<I>(mut self, value: I) -> Self
    where
        I: Into<Option<FilterUnits>>,
    {
        self.filter_units = value.into().unwrap_or_default();
        self
    }

    pub fn primitive_units<I>(mut self, value: I) -> Self
    where
        I: Into<Option<PrimitiveUnits>>,
    {
        self.primitive_units = value.into().unwrap_or_default();
        self
    }
}

impl HasFilterRegion for Filter {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl FilterRegionConfig for Filter {}
impl IsDefault for Filter {}
impl ResourceIri for Filter {}

impl Display for Filter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "filter")?
            .write(|out| self.region.fmt(out))?
            .attr("id", (self.iri(),))?
            .attr_if(
                "filterUnits",
                (&self.filter_units,),
                !self.filter_units.is_default(),
            )?
            .attr_if(
                "primitiveUnits",
                (&self.primitive_units,),
                !self.primitive_units.is_default(),
            )?
            .content(|out| {
                self.primitives
                    .iter()
                    .try_for_each(|primitive| primitive.fmt(out))
            })?
            .close()
    }
}
