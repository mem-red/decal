use crate::filters::primitives::PrimitiveBuilder;
use crate::macros::ff32;
use crate::paint::ResourceIri;
use crate::primitives::FilterInput;
use crate::utils::{FloatWriter, IsDefault};
use enum_display::EnumDisplay;
use std::fmt::{Display, Formatter, Write};
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

    pub fn arithmetic<T>(k1: T, k2: T, k3: T, k4: T) -> Self
    where
        T: Into<f32>,
    {
        Self(CompositeOperatorInner::Arithmetic {
            k1: ff32!(k1),
            k2: ff32!(k2),
            k3: ff32!(k3),
            k4: ff32!(k4),
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
pub struct Composite {
    input: Option<FilterInput>,
    input2: Option<FilterInput>,
    operator: CompositeOperatorInner,
}

impl Composite {
    pub(crate) fn new() -> Self {
        Composite::default()
    }
}

impl ResourceIri for Composite {}

impl Display for Composite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<feComposite")?;

        if let Some(input) = self.input {
            write!(f, r#" in="{input}""#)?;
        }

        if let Some(input2) = self.input2 {
            write!(f, r#" in2="{input2}""#)?;
        }

        if !self.operator.is_default() {
            write!(f, r#" operator="{}""#, self.operator)?;

            if let CompositeOperatorInner::Arithmetic { k1, k2, k3, k4 } = self.operator {
                f.write_str(r#" k1=""#)?;
                f.write_float(k1.get())?;
                f.write_str(r#"" k2=""#)?;
                f.write_float(k2.get())?;
                f.write_str(r#"" k3=""#)?;
                f.write_float(k3.get())?;
                f.write_str(r#"" k4=""#)?;
                f.write_float(k4.get())?;
                f.write_char('"')?;
            }
        }

        write!(f, r#" result="{}" />"#, self.iri())
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
}
