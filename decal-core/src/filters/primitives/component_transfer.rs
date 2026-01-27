use crate::{
    filters::{
        FilterRegion,
        HasFilterRegion,
        primitives::PrimitiveBuilder,
    },
    macros::{
        ff32,
        nf32,
    },
    paint::ResourceIri,
    primitives::{
        ColorInterpolation,
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
    Write,
};
use strict_num::{
    FiniteF32,
    NormalizedF32,
};

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

/// The channel component transfer function for the [`ComponentTransfer`] filter
/// primitive.
#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub struct TransferFunction(TransferFunctionInner);

impl TransferFunction {
    /// Returns an identity transfer function.
    ///
    /// # Returns
    /// - [`Self`]
    pub const fn identity() -> Self {
        Self(TransferFunctionInner::Identity)
    }

    /// Creates a table-based transfer function.
    ///
    /// # Arguments
    /// - `values`: The table values.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn table(values: Vec<f32>) -> Self {
        Self(TransferFunctionInner::Table(
            values.into_iter().map(|x| nf32!(x)).collect(),
        ))
    }

    /// Creates a discrete transfer function.
    ///
    /// # Arguments
    /// - `values`: The discrete values.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn discrete(values: Vec<f32>) -> Self {
        Self(TransferFunctionInner::Discrete(
            values.into_iter().map(|x| nf32!(x)).collect(),
        ))
    }

    /// Creates a linear transfer function.
    ///
    /// # Arguments
    /// - `slope`: The linear slope.
    /// - `intercept`: The linear intercept.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn linear(slope: f32, intercept: f32) -> Self {
        Self(TransferFunctionInner::Linear {
            slope: ff32!(slope, 1.0),
            intercept: ff32!(intercept),
        })
    }

    /// Creates a gamma transfer function.
    ///
    /// # Arguments
    /// - `amplitude`: The gamma amplitude.
    /// - `exponent`: The gamma exponent.
    /// - `offset`: The gamma offset.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn gamma(amplitude: f32, exponent: f32, offset: f32) -> Self {
        Self(TransferFunctionInner::Gamma {
            amplitude: ff32!(amplitude, 1.0),
            exponent: ff32!(exponent, 1.0),
            offset: ff32!(offset),
        })
    }

    /// Serializes the transfer function.
    fn serialize<T>(&self, out: &mut T, element_name: &str) -> std::fmt::Result
    where
        T: Write,
    {
        self.0.serialize(out, element_name)
    }
}

/// The component transfer filter primitive.
#[derive(Debug, Hash, Eq, PartialEq, Clone, SmartDefault)]
pub struct ComponentTransfer {
    input: Option<FilterInput>,
    func_r: TransferFunction,
    func_g: TransferFunction,
    func_b: TransferFunction,
    func_a: TransferFunction,
    region: FilterRegion,
    #[default(ColorInterpolation::LinearRgb)]
    color_interpolation: ColorInterpolation,
}

impl ComponentTransfer {
    /// Creates a new [`ComponentTransfer`] primitive.
    ///
    /// # Returns
    /// - [`Self`]
    pub(crate) fn new() -> Self {
        ComponentTransfer::default()
    }
}

impl HasFilterRegion for ComponentTransfer {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for ComponentTransfer {}

impl Display for ComponentTransfer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feComponentTransfer")?
            .write(|out| self.region.fmt(out))?
            .attr("in", self.input.map(|x| (x,)))?
            .attr_if(
                "color-interpolation-filters",
                (&self.color_interpolation,),
                self.color_interpolation != ColorInterpolation::LinearRgb,
            )?
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
    /// Sets the input for the component transfer operation.
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

    /// Sets the red channel transfer function.
    ///
    /// # Arguments
    /// - `func`: The [`TransferFunction`] applied to the red channel.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn func_r(mut self, func: TransferFunction) -> Self {
        self.inner.func_r = func;
        self
    }

    /// Sets the green channel transfer function.
    ///
    /// # Arguments
    /// - `func`: The [`TransferFunction`] applied to the green channel.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn func_g(mut self, func: TransferFunction) -> Self {
        self.inner.func_g = func;
        self
    }

    /// Sets the blue channel transfer function.
    ///
    /// # Arguments
    /// - `func`: The [`TransferFunction`] applied to the blue channel.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn func_b(mut self, func: TransferFunction) -> Self {
        self.inner.func_b = func;
        self
    }

    /// Sets the alpha channel transfer function.
    ///
    /// # Arguments
    /// - `func`: The [`TransferFunction`] applied to the alpha channel.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn func_a(mut self, func: TransferFunction) -> Self {
        self.inner.func_a = func;
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
        ctx.component_transfer()
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
<feComponentTransfer
    x="0.5"
    y="0.6"
    width="110"
    height="120"
    result="{}">
</feComponentTransfer>
"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders() {
        let ctx = FilterContext::default();
        ctx.component_transfer().finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feComponentTransfer result="{}"></feComponentTransfer>"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_with_transfer_functions() {
        let ctx = FilterContext::default();
        ctx.component_transfer()
            .func_r(TransferFunction::table(vec![0.01, 0.02, 0.03]))
            .func_g(TransferFunction::table(vec![0.11, 0.12, 0.13]))
            .func_b(TransferFunction::table(vec![0.21, 0.22, 0.23]))
            .func_a(TransferFunction::table(vec![0.31, 0.32, 0.33]))
            .finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feComponentTransfer result="{}">
    <feFuncR type="table" tableValues="0.01 0.02 0.03" />
    <feFuncG type="table" tableValues="0.11 0.12 0.13" />
    <feFuncB type="table" tableValues="0.21 0.22 0.23" />
    <feFuncA type="table" tableValues="0.31 0.32 0.33" />
</feComponentTransfer>
"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn does_not_render_identity_function() {
        let ctx = FilterContext::default();
        ctx.component_transfer()
            .func_r(TransferFunction::identity())
            .finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feComponentTransfer result="{}"></feComponentTransfer>"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_with_table_function() {
        let ctx = FilterContext::default();
        ctx.component_transfer()
            .func_r(TransferFunction::table(vec![0.1, 0.2, 0.3]))
            .finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feComponentTransfer result="{}">
    <feFuncR type="table" tableValues="0.1 0.2 0.3" />
</feComponentTransfer>
"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_with_discrete_function() {
        let ctx = FilterContext::default();
        ctx.component_transfer()
            .func_r(TransferFunction::discrete(vec![0.1, 0.2]))
            .finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feComponentTransfer result="{}">
    <feFuncR type="discrete" tableValues="0.1 0.2" />
</feComponentTransfer>
"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_with_linear_function() {
        let ctx = FilterContext::default();
        ctx.component_transfer()
            .func_r(TransferFunction::linear(0.2, 0.5))
            .finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feComponentTransfer result="{}">
    <feFuncR type="linear" slope="0.2" intercept="0.5" />
</feComponentTransfer>
"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_with_gamma_function() {
        let ctx = FilterContext::default();
        ctx.component_transfer()
            .func_r(TransferFunction::gamma(1.2, 0.5, 0.2))
            .finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feComponentTransfer result="{}">
    <feFuncR type="gamma" amplitude="1.2" exponent="0.5" offset="0.2" />
</feComponentTransfer>
"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_with_attrs() {
        let ctx = FilterContext::default();
        let input = FilterInput::source_graphic();
        let color_interpolation = ColorInterpolation::SRgb;
        ctx.component_transfer()
            .input(input)
            .color_interpolation(color_interpolation)
            .finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feComponentTransfer
    in="{input}"
    color-interpolation-filters="{color_interpolation}"
    result="{}">
</feComponentTransfer>
"#,
                node.iri()
            ),
        );
    }
}
