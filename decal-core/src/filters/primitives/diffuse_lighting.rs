use crate::filters::primitives::PrimitiveBuilder;
use crate::filters::{FilterRegion, HasFilterRegion};
use crate::macros::{ff32, pf32};
use crate::paint::ResourceIri;
use crate::primitives::{Color, FilterInput, LightSource};
use crate::utils::ElementWriter;
use std::fmt::{Display, Formatter};
use strict_num::{FiniteF32, PositiveF32};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct DiffuseLighting {
    input: Option<FilterInput>,
    light_source: LightSource,
    lighting_color: Option<Color>,
    surface_scale: FiniteF32,
    diffuse_constant: PositiveF32,
    region: FilterRegion,
}

impl DiffuseLighting {
    pub(crate) fn new(light_source: LightSource) -> Self {
        DiffuseLighting {
            input: None,
            light_source,
            lighting_color: None,
            surface_scale: ff32!(1.0),
            diffuse_constant: pf32!(1.0),
            region: Default::default(),
        }
    }
}

impl HasFilterRegion for DiffuseLighting {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for DiffuseLighting {}

impl Display for DiffuseLighting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feDiffuseLighting")?
            .write(|out| self.region.fmt(out))?
            .attr("in", self.input.map(|x| (x,)))?
            .attr_if(
                "surfaceScale",
                self.surface_scale,
                self.surface_scale.get() != 1.0,
            )?
            .attr_if(
                "diffuseConstant",
                self.diffuse_constant,
                self.diffuse_constant.get() != 1.0,
            )?
            .attr("lighting-color", self.lighting_color.map(|x| (x,)))?
            .attr("result", (self.iri(),))?
            .content(|out| self.light_source.fmt(out))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, DiffuseLighting> {
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input = Some(input.into());
        self
    }

    pub fn lighting_color<T>(mut self, color: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.inner.lighting_color = color.into();
        self
    }

    pub fn surface_scale(mut self, scale: f32) -> Self {
        self.inner.surface_scale = ff32!(scale);
        self
    }

    pub fn diffuse_constant(mut self, constant: f32) -> Self {
        self.inner.diffuse_constant = pf32!(constant);
        self
    }
}
