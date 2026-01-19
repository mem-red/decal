use crate::{
    filters::{
        FilterRegion,
        HasFilterRegion,
        primitives::PrimitiveBuilder,
    },
    paint::ResourceIri,
    primitives::{
        ColorInterpolation,
        FilterInput,
    },
    utils::ElementWriter,
};
use smart_default::SmartDefault;
use std::fmt::{
    Display,
    Formatter,
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, SmartDefault)]
pub struct Merge {
    inputs: Vec<FilterInput>,
    region: FilterRegion,
    #[default(ColorInterpolation::LinearRgb)]
    color_interpolation: ColorInterpolation,
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
            .attr_if(
                "color-interpolation-filters",
                (&self.color_interpolation,),
                self.color_interpolation != ColorInterpolation::LinearRgb,
            )?
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

    pub fn color_interpolation(mut self, value: ColorInterpolation) -> Self {
        self.inner.color_interpolation = value;
        self
    }
}
