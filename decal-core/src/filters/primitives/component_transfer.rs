use crate::filters::primitives::PrimitiveBuilder;
use crate::filters::{FilterRegion, HasFilterRegion};
use crate::macros::{ff32, nf32};
use crate::paint::ResourceIri;
use crate::primitives::FilterInput;
use crate::utils::{ElementWriter, FloatWriter, IsDefault, write_spaced};
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

        let func_el = ElementWriter::new(out, element_name)?;

        match self {
            Self::Table(values) => {
                func_el
                    .attr("type", "table")?
                    .write_attr("tableValues", |out| {
                        write_spaced(out, values.iter(), |out, value| {
                            out.write_float(value.get())
                        })
                    })
            }
            Self::Discrete(values) => {
                func_el
                    .attr("type", "discrete")?
                    .write_attr("tableValues", |out| {
                        write_spaced(out, values.iter(), |out, value| {
                            out.write_float(value.get())
                        })
                    })
            }
            Self::Linear { slope, intercept } => func_el
                .attr("type", "linear")?
                .attrs([("slope", *slope), ("intercept", *intercept)]),
            Self::Gamma {
                amplitude,
                exponent,
                offset,
            } => func_el.attr("type", "gamma")?.attrs([
                ("amplitude", *amplitude),
                ("exponent", *exponent),
                ("offset", *offset),
            ]),
            _ => unreachable!(),
        }?
        .close()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
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

    pub fn linear(slope: f32, intercept: f32) -> Self {
        Self(TransferFunctionInner::Linear {
            slope: ff32!(slope, 1.0),
            intercept: ff32!(intercept),
        })
    }

    pub fn gamma(amplitude: f32, exponent: f32, offset: f32) -> Self {
        Self(TransferFunctionInner::Gamma {
            amplitude: ff32!(amplitude, 1.0),
            exponent: ff32!(exponent, 1.0),
            offset: ff32!(offset),
        })
    }

    //

    fn serialize<T>(&self, out: &mut T, element_name: &str) -> std::fmt::Result
    where
        T: Write,
    {
        self.0.serialize(out, element_name)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub struct ComponentTransfer {
    input: Option<FilterInput>,
    func_r: TransferFunction,
    func_g: TransferFunction,
    func_b: TransferFunction,
    func_a: TransferFunction,
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
        ElementWriter::new(f, "feComponentTransfer")?
            .write(|out| self.region.fmt(out))?
            .attr("in", self.input.map(|x| (x,)))?
            .attr("result", (self.iri(),))?
            .content(|out| {
                self.func_r.serialize(out, "feFuncR")?;
                self.func_g.serialize(out, "feFuncG")?;
                self.func_b.serialize(out, "feFuncB")?;
                self.func_a.serialize(out, "feFuncA")?;

                Ok(())
            })?
            .close()
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
        self.inner.func_r = func;
        self
    }

    pub fn func_g(mut self, func: TransferFunction) -> Self {
        self.inner.func_g = func;
        self
    }

    pub fn func_b(mut self, func: TransferFunction) -> Self {
        self.inner.func_b = func;
        self
    }

    pub fn func_a(mut self, func: TransferFunction) -> Self {
        self.inner.func_a = func;
        self
    }
}
