use crate::{
    filters::{
        FilterRegion,
        FilterRegionConfig,
        HasFilterRegion,
        context::FilterContext,
        primitives::FilterPrimitive,
    },
    paint::ResourceIri,
    primitives::{
        ColorInterpolation,
        FilterUnits,
        PrimitiveUnits,
    },
    utils::{
        ElementWriter,
        IsDefault,
    },
};
use smart_default::SmartDefault;
use std::{
    fmt::{
        Display,
        Formatter,
    },
    hash::Hash,
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, SmartDefault)]
pub struct Filter {
    filter_units: FilterUnits,
    primitive_units: PrimitiveUnits,
    primitives: Vec<FilterPrimitive>,
    region: FilterRegion,
    #[default(ColorInterpolation::LinearRgb)]
    color_interpolation: ColorInterpolation,
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

    //

    fn append(mut self, next: Filter) -> Self {
        self.filter_units = next.filter_units;
        self.primitive_units = next.primitive_units;
        self.region = next.region;
        self.color_interpolation = next.color_interpolation;
        self.primitives.extend(next.primitives);

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
            .attr_if(
                "color-interpolation-filters",
                (&self.color_interpolation,),
                self.color_interpolation != ColorInterpolation::LinearRgb,
            )?
            .content(|out| {
                self.primitives
                    .iter()
                    .try_for_each(|primitive| primitive.fmt(out))
            })?
            .close()
    }
}

//

impl From<Vec<Filter>> for Filter {
    fn from(value: Vec<Filter>) -> Self {
        value
            .into_iter()
            .fold(Filter::default(), |acc, next| acc.append(next))
    }
}

impl<const N: usize> From<[Filter; N]> for Filter {
    fn from(value: [Filter; N]) -> Self {
        value
            .into_iter()
            .fold(Filter::default(), |acc, next| acc.append(next))
    }
}
