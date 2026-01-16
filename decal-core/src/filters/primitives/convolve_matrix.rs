use crate::filters::primitives::PrimitiveBuilder;
use crate::filters::{FilterRegion, HasFilterRegion};
use crate::macros::ff32;
use crate::paint::ResourceIri;
use crate::primitives::EdgeMode;
use crate::primitives::{ColorInterpolation, FilterInput};
use crate::utils::{ElementWriter, FloatWriter, IsDefault, write_spaced};
use smart_default::SmartDefault;
use std::fmt::{Display, Formatter};
use strict_num::FiniteF32;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Order(u32, u32);

impl Default for Order {
    fn default() -> Self {
        Self(3, 3)
    }
}

impl IsDefault for Order {}

impl From<u32> for Order {
    fn from(value: u32) -> Self {
        Self(value, value)
    }
}

impl From<(u32, u32)> for Order {
    fn from((x, y): (u32, u32)) -> Self {
        Self(x, y)
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;

        if self.0 != self.1 {
            write!(f, " {}", self.1)?;
        }

        Ok(())
    }
}

//

#[derive(Debug, Hash, Eq, PartialEq, Clone, SmartDefault)]
pub struct ConvolveMatrix {
    input: Option<FilterInput>,
    order: Order,
    kernel_matrix: Vec<FiniteF32>,
    divisor: Option<FiniteF32>,
    bias: FiniteF32,
    target_x: Option<u32>,
    target_y: Option<u32>,
    edge_mode: EdgeMode,
    preserve_alpha: bool,
    region: FilterRegion,
    #[default(ColorInterpolation::LinearRgb)]
    color_interpolation: ColorInterpolation,
}

impl ConvolveMatrix {
    pub(crate) fn new(kernel_matrix: Vec<f32>) -> Self {
        ConvolveMatrix {
            kernel_matrix: kernel_matrix.into_iter().map(|x| ff32!(x)).collect(),
            ..Default::default()
        }
    }
}

impl HasFilterRegion for ConvolveMatrix {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for ConvolveMatrix {}

impl Display for ConvolveMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feConvolveMatrix")?
            .write(|out| self.region.fmt(out))?
            .attr("in", self.input.map(|x| (x,)))?
            .attr_if("order", (&self.order,), !self.order.is_default())?
            .write_attr("kernelMatrix", |out| {
                write_spaced(out, self.kernel_matrix.iter(), |out, value| {
                    out.write_float(value.get())
                })
            })?
            .attr("divisor", self.divisor)?
            .attr_if("bias", self.bias, self.bias.get() != 0.0)?
            .attrs([
                ("targetX", self.target_x.map(|x| (x,))),
                ("targetY", self.target_y.map(|y| (y,))),
            ])?
            .attr_if("edgeMode", (self.edge_mode,), !self.edge_mode.is_default())?
            .attr_if("preserveAlpha", "true", self.preserve_alpha)?
            .attr_if(
                "color-interpolation-filters",
                (&self.color_interpolation,),
                self.color_interpolation != ColorInterpolation::LinearRgb,
            )?
            .attr("result", (self.iri(),))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, ConvolveMatrix> {
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input = Some(input.into());
        self
    }

    pub fn order<T>(mut self, order: T) -> Self
    where
        T: Into<Order>,
    {
        self.inner.order = order.into();
        self
    }

    pub fn divisor<T>(mut self, divisor: T) -> Self
    where
        T: Into<Option<f32>>,
    {
        self.inner.divisor = divisor.into().map(|x| ff32!(x));
        self
    }

    pub fn bias(mut self, bias: f32) -> Self {
        self.inner.bias = ff32!(bias);
        self
    }

    pub fn target_x<T>(mut self, target_x: T) -> Self
    where
        T: Into<Option<u32>>,
    {
        self.inner.target_x = target_x.into();
        self
    }

    pub fn target_y<T>(mut self, target_y: T) -> Self
    where
        T: Into<Option<u32>>,
    {
        self.inner.target_y = target_y.into();
        self
    }

    pub fn edge_mode(mut self, edge_mode: EdgeMode) -> Self {
        self.inner.edge_mode = edge_mode;
        self
    }

    pub fn preserve_alpha(mut self, value: bool) -> Self {
        self.inner.preserve_alpha = value;
        self
    }

    pub fn color_interpolation(mut self, value: ColorInterpolation) -> Self {
        self.inner.color_interpolation = value;
        self
    }
}
