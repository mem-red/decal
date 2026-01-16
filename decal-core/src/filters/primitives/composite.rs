use crate::filters::primitives::PrimitiveBuilder;
use crate::filters::{FilterRegion, HasFilterRegion};
use crate::macros::ff32;
use crate::paint::ResourceIri;
use crate::prelude::ColorInterpolation;
use crate::primitives::FilterInput;
use crate::utils::{ElementWriter, IsDefault};
use enum_display::EnumDisplay;
use smart_default::SmartDefault;
use std::fmt::{Display, Formatter};
use strict_num::FiniteF32;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
enum CompositeOperatorInner {
    #[default]
    #[display("over")]
    Over,
    #[display("in")]
    In,
    #[display("out")]
    Out,
    #[display("atop")]
    Atop,
    #[display("xor")]
    Xor,
    #[display("lighter")]
    Lighter,
    #[display("arithmetic")]
    Arithmetic {
        k1: FiniteF32,
        k2: FiniteF32,
        k3: FiniteF32,
        k4: FiniteF32,
    },
}

impl IsDefault for CompositeOperatorInner {}

#[derive(Debug, Copy, Clone, Default)]
pub struct CompositeOperator(CompositeOperatorInner);

impl CompositeOperator {
    pub const fn over() -> Self {
        Self(CompositeOperatorInner::Over)
    }

    pub const fn r#in() -> Self {
        Self(CompositeOperatorInner::In)
    }

    pub const fn out() -> Self {
        Self(CompositeOperatorInner::Out)
    }

    pub const fn atop() -> Self {
        Self(CompositeOperatorInner::Atop)
    }

    pub const fn xor() -> Self {
        Self(CompositeOperatorInner::Xor)
    }

    pub const fn lighter() -> Self {
        Self(CompositeOperatorInner::Lighter)
    }

    pub fn arithmetic(k1: f32, k2: f32, k3: f32, k4: f32) -> Self {
        Self(CompositeOperatorInner::Arithmetic {
            k1: ff32!(k1),
            k2: ff32!(k2),
            k3: ff32!(k3),
            k4: ff32!(k4),
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, SmartDefault)]
pub struct Composite {
    input: Option<FilterInput>,
    input2: Option<FilterInput>,
    operator: CompositeOperatorInner,
    region: FilterRegion,
    #[default(ColorInterpolation::LinearRgb)]
    color_interpolation: ColorInterpolation,
}

impl Composite {
    pub(crate) fn new() -> Self {
        Composite::default()
    }
}

impl ResourceIri for Composite {}

impl HasFilterRegion for Composite {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl Display for Composite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut composite = ElementWriter::new(f, "feComposite")?
            .write(|out| self.region.fmt(out))?
            .attrs([
                ("in", self.input.map(|x| (x,))),
                ("in2", self.input2.map(|x| (x,))),
            ])?
            .attr_if("operator", (self.operator,), !self.operator.is_default())?;

        if let CompositeOperatorInner::Arithmetic { k1, k2, k3, k4 } = self.operator {
            composite = composite.attrs([("k1", k1), ("k2", k2), ("k3", k3), ("k4", k4)])?;
        }

        composite
            .attr_if(
                "color-interpolation-filters",
                (&self.color_interpolation,),
                self.color_interpolation != ColorInterpolation::LinearRgb,
            )?
            .attr("result", (self.iri(),))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, Composite> {
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

    pub fn operator(mut self, operator: CompositeOperator) -> Self {
        self.inner.operator = operator.0;
        self
    }

    pub fn color_interpolation(mut self, value: ColorInterpolation) -> Self {
        self.inner.color_interpolation = value;
        self
    }
}
