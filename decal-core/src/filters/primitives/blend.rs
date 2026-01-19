use crate::{
    filters::{
        FilterRegion,
        HasFilterRegion,
        primitives::PrimitiveBuilder,
    },
    paint::ResourceIri,
    primitives::{
        BlendMode,
        ColorInterpolation,
        FilterInput,
    },
    utils::{
        ElementWriter,
        IsDefault,
    },
};
use smart_default::SmartDefault;
use std::fmt::{
    Display,
    Formatter,
};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, SmartDefault)]
pub struct Blend {
    input: Option<FilterInput>,
    input2: Option<FilterInput>,
    mode: BlendMode,
    region: FilterRegion,
    #[default(ColorInterpolation::LinearRgb)]
    color_interpolation: ColorInterpolation,
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
            .attr_if(
                "color-interpolation-filters",
                (&self.color_interpolation,),
                self.color_interpolation != ColorInterpolation::LinearRgb,
            )?
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

    pub fn color_interpolation(mut self, value: ColorInterpolation) -> Self {
        self.inner.color_interpolation = value;
        self
    }
}
