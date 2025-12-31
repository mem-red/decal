use crate::filters::primitives::PrimitiveBuilder;
use crate::filters::{FilterRegion, HasFilterRegion};
use crate::paint::ResourceIri;
use crate::primitives::{EdgeMode, FilterInput, PositiveF32Pair};
use crate::utils::IsDefault;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
pub struct Blur {
    input: Option<FilterInput>,
    std_deviation: PositiveF32Pair,
    edge_mode: EdgeMode,
    region: FilterRegion,
}

impl Blur {
    pub(crate) fn new() -> Self {
        Blur::default()
    }
}

impl ResourceIri for Blur {}

impl HasFilterRegion for Blur {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl Display for Blur {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<feGaussianBlur")?;
        self.region.fmt(f)?;

        if let Some(input) = self.input {
            write!(f, r#" in="{input}""#)?;
        }

        if !self.std_deviation.is_zero() {
            write!(f, r#" stdDeviation="{}""#, self.std_deviation)?;
        }

        if !self.edge_mode.is_default() {
            write!(f, r#" edgeMode="{}""#, self.edge_mode)?;
        }

        write!(f, r#" result="{}" />"#, self.iri())
    }
}

impl<'a> PrimitiveBuilder<'a, Blur> {
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
}
