use crate::{
    filters::{
        FilterRegion,
        HasFilterRegion,
        primitives::PrimitiveBuilder,
    },
    paint::ResourceIri,
    primitives::{
        ColorInterpolation,
        FilterInput,
    },
    utils::ElementWriter,
};
use smart_default::SmartDefault;
use std::fmt::{
    Display,
    Formatter,
};

/// The merge filter primitive.
#[derive(Debug, Hash, Eq, PartialEq, Clone, SmartDefault)]
pub struct Merge {
    inputs: Vec<FilterInput>,
    region: FilterRegion,
    #[default(ColorInterpolation::LinearRgb)]
    color_interpolation: ColorInterpolation,
}

impl Merge {
    /// Creates a new [`Merge`] primitive.
    ///
    /// The primitive produces no output until at least one input is added.
    ///
    /// # Returns
    /// - [`Self`]
    pub(crate) fn new() -> Self {
        Merge::default()
    }
}

impl HasFilterRegion for Merge {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for Merge {}

impl Display for Merge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.inputs.is_empty() {
            return Ok(());
        }

        ElementWriter::new(f, "feMerge")?
            .write(|out| self.region.fmt(out))?
            .attr_if(
                "color-interpolation-filters",
                (&self.color_interpolation,),
                self.color_interpolation != ColorInterpolation::LinearRgb,
            )?
            .attr("result", (self.iri(),))?
            .content(|out| {
                self.inputs.iter().try_for_each(|node| {
                    ElementWriter::new(out, "feMergeNode")?
                        .attr("in", (node,))?
                        .close()
                })
            })?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, Merge> {
    /// Appends a single input to the merge primitive.
    ///
    /// Inputs are merged in the order they are added.
    ///
    /// # Arguments
    /// - `input`: The [`FilterInput`] to append.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.inputs.push(input.into());
        self
    }

    /// Appends multiple inputs to the merge primitive.
    ///
    /// Inputs are merged in the order they are provided.
    ///
    /// # Arguments
    /// - `inputs`: An iterator of [`FilterInput`] values.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn inputs<I, T>(mut self, inputs: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<FilterInput>,
    {
        self.inner.inputs.extend(inputs.into_iter().map(Into::into));
        self
    }

    /// Sets the color interpolation space used when merging inputs.
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
        let input = FilterInput::source_alpha();

        ctx.merge()
            .input(input)
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
<feMerge x="0.5" y="0.6" width="110" height="120" result="{}">
    <feMergeNode in="{input}" />
</feMerge>
"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn does_not_render_when_empty() {
        let ctx = FilterContext::default();
        ctx.merge().finish();
        let node = &ctx.into_primitives()[0];
        assert!(node.to_string().is_empty());
    }

    #[test]
    fn renders_with_single_input() {
        let ctx = FilterContext::default();
        let input = FilterInput::source_graphic();
        ctx.merge().input(input).finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feMerge result="{}">
    <feMergeNode in="{input}" />
</feMerge>
"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_with_multiple_inputs() {
        let ctx = FilterContext::default();

        ctx.merge()
            .input(FilterInput::source_graphic())
            .input(FilterInput::source_alpha())
            .inputs([
                FilterInput::source_graphic(),
                FilterInput::source_alpha(),
                FilterInput::background_image(),
            ])
            .finish();

        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feMerge result="{}">
    <feMergeNode in="{}" />
    <feMergeNode in="{}" />
    <feMergeNode in="{}" />
    <feMergeNode in="{}" />
    <feMergeNode in="{}" />
</feMerge>
"#,
                node.iri(),
                FilterInput::source_graphic(),
                FilterInput::source_alpha(),
                FilterInput::source_graphic(),
                FilterInput::source_alpha(),
                FilterInput::background_image(),
            ),
        );
    }

    #[test]
    fn renders_with_attrs() {
        let ctx = FilterContext::default();
        let input = FilterInput::source_alpha();
        let color_interpolation = ColorInterpolation::SRgb;

        ctx.merge()
            .input(input)
            .color_interpolation(color_interpolation)
            .finish();

        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feMerge color-interpolation-filters="{color_interpolation}" result="{}">
    <feMergeNode in="{input}" />
</feMerge>
"#,
                node.iri()
            ),
        );
    }
}
