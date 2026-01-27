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
        FilterInput,
    },
    utils::{
        ElementWriter,
        IsDefault,
    },
};
use enum_display::EnumDisplay;
use smart_default::SmartDefault;
use std::fmt::{
    Display,
    Formatter,
};
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

/// The composite operator.
#[derive(Debug, Copy, Clone, Default)]
pub struct CompositeOperator(CompositeOperatorInner);

impl CompositeOperator {
    /// Creates an `over` composite operator.
    ///
    /// # Returns
    /// - [`Self`]
    pub const fn over() -> Self {
        Self(CompositeOperatorInner::Over)
    }

    /// Creates an `in` composite operator.
    ///
    /// # Returns
    /// - [`Self`]
    pub const fn r#in() -> Self {
        Self(CompositeOperatorInner::In)
    }

    /// Creates an `out` composite operator.
    ///
    /// # Returns
    /// - [`Self`]
    pub const fn out() -> Self {
        Self(CompositeOperatorInner::Out)
    }

    /// Creates an `atop` composite operator.
    ///
    /// # Returns
    /// - [`Self`]
    pub const fn atop() -> Self {
        Self(CompositeOperatorInner::Atop)
    }

    /// Creates an `xor` composite operator.
    ///
    /// # Returns
    /// - [`Self`]
    pub const fn xor() -> Self {
        Self(CompositeOperatorInner::Xor)
    }

    /// Creates an `lighter` composite operator.
    ///
    /// # Returns
    /// - [`Self`]
    pub const fn lighter() -> Self {
        Self(CompositeOperatorInner::Lighter)
    }

    /// Creates an `arithmetic` composite operator.
    ///
    /// # Arguments
    /// - `k1`: The first arithmetic coefficient.
    /// - `k2`: The second arithmetic coefficient.
    /// - `k3`: The third arithmetic coefficient.
    /// - `k4`: The fourth arithmetic coefficient.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn arithmetic(k1: f32, k2: f32, k3: f32, k4: f32) -> Self {
        Self(CompositeOperatorInner::Arithmetic {
            k1: ff32!(k1),
            k2: ff32!(k2),
            k3: ff32!(k3),
            k4: ff32!(k4),
        })
    }
}

/// The composite filter primitive.
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
    /// Creates a new [`Composite`] primitive.
    ///
    /// # Returns
    /// - [`Self`]
    pub(crate) fn new() -> Self {
        Composite::default()
    }
}

impl HasFilterRegion for Composite {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for Composite {}

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
    /// Sets the first input for the composite operation.
    ///
    /// # Arguments
    /// - `input`: The [`FilterInput`] used as the first operand.
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

    /// Sets the second input for the composite operation.
    ///
    /// # Arguments
    /// - `input2`: The [`FilterInput`] used as the second operand.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn input2<T>(mut self, input2: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input2 = Some(input2.into());
        self
    }

    /// Sets the composite operator.
    ///
    /// # Arguments
    /// - `operator`: The [`CompositeOperator`] defining how inputs are
    ///   combined.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn operator(mut self, operator: CompositeOperator) -> Self {
        self.inner.operator = operator.0;
        self
    }

    /// Sets the color interpolation space used during compositing.
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
        ctx.composite()
            .x(0.5)
            .y(0.6)
            .width(110)
            .height(120)
            .finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feComposite x="0.5" y="0.6" width="110" height="120" result="{}" />"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders() {
        let ctx = FilterContext::default();
        ctx.composite().finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(r#"<feComposite result="{}" />"#, node.iri()),
        );
    }

    #[test]
    fn renders_with_arithmetic_operator() {
        let ctx = FilterContext::default();
        ctx.composite()
            .operator(CompositeOperator::arithmetic(0.1, 0.2, 0.3, 0.4))
            .finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feComposite
    operator="arithmetic"
    k1="0.1"
    k2="0.2"
    k3="0.3"
    k4="0.4"
    result="{}"
/>
"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_with_attrs() {
        let ctx = FilterContext::default();
        let input = FilterInput::source_graphic();
        let input2 = FilterInput::source_alpha();
        let operator = CompositeOperator::atop();
        let color_interpolation = ColorInterpolation::SRgb;

        ctx.composite()
            .input(input)
            .input2(input2)
            .operator(operator)
            .color_interpolation(color_interpolation)
            .finish();

        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feComposite
    in="{input}"
    in2="{input2}"
    operator="{}"
    color-interpolation-filters="{color_interpolation}"
    result="{}"
/>
"#,
                operator.0,
                node.iri()
            ),
        );
    }
}
