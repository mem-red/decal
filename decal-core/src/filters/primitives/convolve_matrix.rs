use crate::{
    filters::{
        FilterRegion,
        HasFilterRegion,
        primitives::PrimitiveBuilder,
    },
    macros::ff32,
    paint::ResourceIri,
    primitives::{
        ColorInterpolation,
        EdgeMode,
        FilterInput,
    },
    utils::{
        ElementWriter,
        FloatWriter,
        IsDefault,
        write_spaced,
    },
};
use smart_default::SmartDefault;
use std::fmt::{
    Display,
    Formatter,
};
use strict_num::FiniteF32;

/// The kernel order for the [`ConvolveMatrix`] filter primitive.
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Order(u32, u32);

impl Default for Order {
    fn default() -> Self {
        Self(3, 3)
    }
}

impl IsDefault for Order {}

impl From<u32> for Order {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value, value)
    }
}

impl From<(u32, u32)> for Order {
    #[inline]
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

/// The convolve matrix filter primitive.
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
    /// Creates a new [`ConvolveMatrix`] primitive with the provided kernel
    /// matrix.
    ///
    /// # Arguments
    /// - `kernel_matrix`: The convolution kernel values.
    ///
    /// # Returns
    /// - [`Self`]
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
    /// Sets the input for the convolution operation.
    ///
    /// # Arguments
    /// - `input`: The [`FilterInput`] used as the source graphic.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input = Some(input.into());
        self
    }

    /// Sets the kernel order for the convolution.
    ///
    /// # Arguments
    /// - `order`: The convolution [`Order`].
    ///
    /// # Returns
    /// - [`Self`]
    pub fn order<T>(mut self, order: T) -> Self
    where
        T: Into<Order>,
    {
        self.inner.order = order.into();
        self
    }

    /// Sets the divisor applied to the kernel sum.
    ///
    /// # Arguments
    /// - `divisor`: The divisor value.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn divisor<T>(mut self, divisor: T) -> Self
    where
        T: Into<Option<f32>>,
    {
        self.inner.divisor = divisor.into().map(|x| ff32!(x));
        self
    }

    /// Sets the bias added to the convolution result.
    ///
    /// # Arguments
    /// - `bias`: The bias value.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn bias(mut self, bias: f32) -> Self {
        self.inner.bias = ff32!(bias);
        self
    }

    /// Sets the target X coordinate within the kernel.
    ///
    /// # Arguments
    /// - `target_x`: The target X index.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn target_x<T>(mut self, target_x: T) -> Self
    where
        T: Into<Option<u32>>,
    {
        self.inner.target_x = target_x.into();
        self
    }

    /// Sets the target Y coordinate within the kernel.
    ///
    /// # Arguments
    /// - `target_y`: The target Y index.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn target_y<T>(mut self, target_y: T) -> Self
    where
        T: Into<Option<u32>>,
    {
        self.inner.target_y = target_y.into();
        self
    }

    /// Sets the edge handling mode.
    ///
    /// # Arguments
    /// - `edge_mode`: The [`EdgeMode`] to apply.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn edge_mode(mut self, edge_mode: EdgeMode) -> Self {
        self.inner.edge_mode = edge_mode;
        self
    }

    /// Controls whether the alpha channel is preserved.
    ///
    /// # Arguments
    /// - `value`: Whether to preserve the alpha channel.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn preserve_alpha(mut self, value: bool) -> Self {
        self.inner.preserve_alpha = value;
        self
    }

    /// Sets the color interpolation space used during filtering.
    ///
    /// # Arguments
    /// - `value`: The [`ColorInterpolation`] space to apply.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn color_interpolation(mut self, value: ColorInterpolation) -> Self {
        self.inner.color_interpolation = value;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        filters::{
            FilterContext,
            FilterRegionConfig,
        },
        test_utils::assert_xml,
    };

    #[test]
    fn renders_with_filter_region() {
        let ctx = FilterContext::default();
        ctx.convolve_matrix(vec![0.0])
            .x(0.5)
            .y(0.6)
            .width(110)
            .height(120)
            .finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feConvolveMatrix
    x="0.5"
    y="0.6"
    width="110"
    height="120"
    kernelMatrix="0"
    result="{}"
/>
"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders() {
        let ctx = FilterContext::default();
        ctx.convolve_matrix(vec![0.0, 0.1, 0.2]).finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feConvolveMatrix kernelMatrix="0 0.1 0.2" result="{}" />"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_with_attrs() {
        let ctx = FilterContext::default();
        let input = FilterInput::source_graphic();
        let edge_mode = EdgeMode::Wrap;
        let color_interpolation = ColorInterpolation::SRgb;

        ctx.convolve_matrix(vec![0.0])
            .input(input)
            .order(4)
            .divisor(1.2)
            .bias(1.5)
            .target_x(5)
            .target_y(10)
            .edge_mode(edge_mode)
            .preserve_alpha(true)
            .color_interpolation(color_interpolation)
            .finish();

        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feConvolveMatrix
    kernelMatrix="0"
    in="{input}"
    order="4"
    divisor="1.2"
    bias="1.5"
    targetX="5"
    targetY="10"
    edgeMode="{edge_mode}"
    preserveAlpha="true"
    color-interpolation-filters="{color_interpolation}"
    result="{}"
/>
"#,
                node.iri()
            ),
        );
    }

    //

    #[test]
    fn order_defaults_to_3() {
        assert_eq!(Order::default().to_string(), "3");
    }

    #[test]
    fn order_from_single_value() {
        assert_eq!(Order::from(5).to_string(), "5");
    }

    #[test]
    fn order_from_value_pair() {
        assert_eq!(Order::from((1, 2)).to_string(), "1 2");
    }
}
