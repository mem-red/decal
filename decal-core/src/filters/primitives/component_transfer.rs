use crate::filters::primitives::PrimitiveBuilder;
use crate::filters::{FilterRegion, HasFilterRegion};
use crate::macros::{ff32, nf32};
use crate::paint::ResourceIri;
use crate::primitives::FilterInput;
use crate::utils::{FloatWriter, IsDefault};
use std::fmt::{Display, Formatter, Write};
use strict_num::{FiniteF32, NormalizedF32};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
enum TransferFunctionInner {
    #[default]
    Identity,
    Table(Vec<NormalizedF32>),
    Discrete(Vec<NormalizedF32>),
    Linear {
        slope: FiniteF32,
        intercept: FiniteF32,
    },
    Gamma {
        amplitude: FiniteF32,
        exponent: FiniteF32,
        offset: FiniteF32,
    },
}

impl IsDefault for TransferFunctionInner {}

impl TransferFunctionInner {
    fn serialize<T>(&self, out: &mut T, element_name: &str) -> std::fmt::Result
    where
        T: Write,
    {
        if self.is_default() {
            return Ok(());
        }

        write!(out, "<{element_name}")?;

        match self {
            Self::Table(values) => {
                write!(out, r#" type="table" tableValues=""#)?;
                let mut first = true;

                for value in values {
                    if !first {
                        out.write_char(' ')?;
                    }

                    out.write_float(value.get())?;
                    first = false;
                }

                write!(out, r#"""#)?;
            }
            Self::Discrete(values) => {
                write!(out, r#" type="discrete" tableValues=""#)?;
                let mut first = true;

                for value in values {
                    if !first {
                        out.write_char(' ')?;
                    }

                    out.write_float(value.get())?;
                    first = false;
                }

                write!(out, r#"""#)?;
            }
            Self::Linear { slope, intercept } => {
                out.write_str(r#" type="linear" slope=""#)?;
                out.write_float(slope.get())?;
                out.write_str(r#"" intercept=""#)?;
                out.write_float(intercept.get())?;
                out.write_char('"')?;
            }
            Self::Gamma {
                amplitude,
                exponent,
                offset,
            } => {
                out.write_str(r#" type="gamma" amplitude=""#)?;
                out.write_float(amplitude.get())?;
                out.write_str(r#"" exponent=""#)?;
                out.write_float(exponent.get())?;
                out.write_str(r#"" offset=""#)?;
                out.write_float(offset.get())?;
                out.write_char('"')?;
            }
            _ => unreachable!(),
        }

        out.write_str(" />")
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransferFunction(TransferFunctionInner);

impl TransferFunction {
    pub const fn identity() -> Self {
        Self(TransferFunctionInner::Identity)
    }

    pub fn table(values: Vec<f32>) -> Self {
        Self(TransferFunctionInner::Table(
            values.into_iter().map(|x| nf32!(x)).collect(),
        ))
    }

    pub fn discrete(values: Vec<f32>) -> Self {
        Self(TransferFunctionInner::Discrete(
            values.into_iter().map(|x| nf32!(x)).collect(),
        ))
    }

    pub fn linear<T>(slope: T, intercept: T) -> Self
    where
        T: Into<f32>,
    {
        Self(TransferFunctionInner::Linear {
            slope: ff32!(slope, 1.0),
            intercept: ff32!(intercept),
        })
    }

    pub fn gamma<T>(amplitude: T, exponent: T, offset: T) -> Self
    where
        T: Into<f32>,
    {
        Self(TransferFunctionInner::Gamma {
            amplitude: ff32!(amplitude, 1.0),
            exponent: ff32!(exponent, 1.0),
            offset: ff32!(offset),
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub struct ComponentTransfer {
    input: Option<FilterInput>,
    func_r: TransferFunctionInner,
    func_g: TransferFunctionInner,
    func_b: TransferFunctionInner,
    func_a: TransferFunctionInner,
    region: FilterRegion,
}

impl ComponentTransfer {
    pub(crate) fn new() -> Self {
        ComponentTransfer::default()
    }
}

impl ResourceIri for ComponentTransfer {}

impl HasFilterRegion for ComponentTransfer {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl Display for ComponentTransfer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<feComponentTransfer")?;
        self.region.fmt(f)?;

        if let Some(input) = self.input {
            write!(f, r#" in="{input}""#)?;
        }

        write!(f, r#" result="{}">"#, self.iri())?;

        self.func_r.serialize(f, "feFuncR")?;
        self.func_g.serialize(f, "feFuncG")?;
        self.func_b.serialize(f, "feFuncB")?;
        self.func_a.serialize(f, "feFuncA")?;

        write!(f, "</feComponentTransfer>")
    }
}

impl<'a> PrimitiveBuilder<'a, ComponentTransfer> {
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input = Some(input.into());
        self
    }

    pub fn source_graphic(mut self) -> Self {
        self.inner.input = Some(FilterInput::source_graphic());
        self
    }

    pub fn source_alpha(mut self) -> Self {
        self.inner.input = Some(FilterInput::source_alpha());
        self
    }

    pub fn func_r(mut self, func: TransferFunction) -> Self {
        self.inner.func_r = func.0;
        self
    }

    pub fn func_g(mut self, func: TransferFunction) -> Self {
        self.inner.func_g = func.0;
        self
    }

    pub fn func_b(mut self, func: TransferFunction) -> Self {
        self.inner.func_b = func.0;
        self
    }

    pub fn func_a(mut self, func: TransferFunction) -> Self {
        self.inner.func_a = func.0;
        self
    }
}
