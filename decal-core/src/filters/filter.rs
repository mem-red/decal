use crate::filters::context::FilterContext;
use crate::filters::primitives::FilterPrimitive;
use crate::paint::ResourceIri;
use crate::primitives::{FilterUnits, PrimitiveUnits};
use crate::utils::IsDefault;
use std::fmt::{Display, Formatter, Write};
use std::hash::Hash;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub struct Filter {
    filter_units: FilterUnits,
    primitive_units: PrimitiveUnits,
    primitives: Vec<FilterPrimitive>,
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
                ctx.primitives()
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

    pub fn append(mut self, other: Filter) -> Self {
        self.primitives.extend(other.primitives);
        self
    }
}

impl IsDefault for Filter {}
impl ResourceIri for Filter {}

impl Display for Filter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"<filter id="{}""#, self.iri())?;

        if !self.filter_units.is_default() {
            write!(f, r#" filterUnits="{}""#, self.filter_units)?;
        }

        if !self.primitive_units.is_default() {
            write!(f, r#" primitiveUnits="{}""#, self.primitive_units)?;
        }

        f.write_char('>')?;

        for primitive in &self.primitives {
            write!(f, "{primitive}")?;
        }

        write!(f, r#"</filter>"#)
    }
}
