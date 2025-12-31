use crate::filters::primitives::PrimitiveBuilder;
use crate::filters::{FilterRegion, HasFilterRegion};
use crate::macros::{ff32, pf32};
use crate::paint::ResourceIri;
use crate::primitives::{Color, FilterInput, LightSource, PositiveF32Pair};
use crate::utils::FloatWriter;
use std::fmt::{Display, Formatter, Write};
use strict_num::{FiniteF32, PositiveF32};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct SpecularLighting {
    input: Option<FilterInput>,
    light_source: LightSource,
    lighting_color: Option<Color>,
    surface_scale: FiniteF32,
    specular_constant: PositiveF32,
    specular_exponent: PositiveF32,
    kernel_unit_length: Option<PositiveF32Pair>,
    region: FilterRegion,
}

impl SpecularLighting {
    pub(crate) fn new(light_source: LightSource) -> Self {
        SpecularLighting {
            input: None,
            light_source,
            lighting_color: None,
            surface_scale: ff32!(1.0),
            specular_constant: pf32!(1.0),
            specular_exponent: pf32!(1.0),
            kernel_unit_length: None,
            region: Default::default(),
        }
    }
}

impl HasFilterRegion for SpecularLighting {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for SpecularLighting {}

impl Display for SpecularLighting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<feSpecularLighting")?;
        self.region.fmt(f)?;

        if let Some(input) = self.input {
            write!(f, r#" in="{input}""#)?;
        }

        if self.surface_scale.get() != 1.0 {
            f.write_str(r#" surfaceScale=""#)?;
            f.write_float(self.surface_scale.get())?;
            f.write_char('"')?;
        }

        if self.specular_constant.get() != 1.0 {
            f.write_str(r#" specularConstant=""#)?;
            f.write_float(self.specular_constant.get())?;
            f.write_char('"')?;
        }

        if self.specular_exponent.get() != 1.0 {
            f.write_str(r#" specularExponent=""#)?;
            f.write_float(self.specular_exponent.get())?;
            f.write_char('"')?;
        }

        if let Some(value) = self.kernel_unit_length {
            write!(f, r#" kernelUnitLength="{value}""#)?;
        }

        if let Some(color) = self.lighting_color {
            write!(f, r#" lighting-color="{color}""#)?;
        }

        write!(f, r#" result="{}">"#, self.iri())?;
        self.light_source.fmt(f)?;
        f.write_str("</feSpecularLighting>")
    }
}

impl<'a> PrimitiveBuilder<'a, SpecularLighting> {
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

    pub fn specular_constant(mut self, constant: f32) -> Self {
        self.inner.specular_constant = pf32!(constant);
        self
    }

    pub fn specular_exponent(mut self, exponent: f32) -> Self {
        self.inner.specular_exponent = pf32!(exponent);
        self
    }

    pub fn kernel_unit_length<T>(mut self, value: T) -> Self
    where
        T: Into<Option<PositiveF32Pair>>,
    {
        self.inner.kernel_unit_length = value.into();
        self
    }
}
