use crate::{
    filters::{
        FilterRegion,
        HasFilterRegion,
        primitives::PrimitiveBuilder,
    },
    macros::{
        ff32,
        pf32,
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
use enum_display::EnumDisplay;
use smart_default::SmartDefault;
use std::fmt::{
    Display,
    Formatter,
};
use strict_num::{
    FiniteF32,
    PositiveF32,
};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, EnumDisplay)]
enum ColorMatrixType {
    Matrix([[FiniteF32; 5]; 4]),
    Saturate(PositiveF32),
    HueRotate(FiniteF32),
    LuminanceToAlpha,
}

impl IsDefault for ColorMatrixType {}

impl Default for ColorMatrixType {
    fn default() -> Self {
        Self::identity()
    }
}

impl ColorMatrixType {
    fn matrix(matrix: [[f32; 5]; 4]) -> Self {
        let mut ff32_matrix: [[FiniteF32; 5]; 4] = [[FiniteF32::default(); 5]; 4];

        for (i, row) in matrix.iter().enumerate() {
            for (j, element) in row.iter().enumerate() {
                ff32_matrix[i][j] = ff32!(*element);
            }
        }

        Self::Matrix(ff32_matrix)
    }

    fn identity() -> Self {
        Self::matrix([
            [1.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0, 0.0],
        ])
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, SmartDefault)]
pub struct ColorMatrix {
    input: Option<FilterInput>,
    kind: ColorMatrixType,
    region: FilterRegion,
    #[default(ColorInterpolation::LinearRgb)]
    color_interpolation: ColorInterpolation,
}

impl ColorMatrix {
    pub(crate) fn new() -> Self {
        ColorMatrix::default()
    }
}

impl HasFilterRegion for ColorMatrix {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for ColorMatrix {}

impl Display for ColorMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut color_matrix = ElementWriter::new(f, "feColorMatrix")?
            .write(|out| self.region.fmt(out))?
            .attr("in", self.input.map(|x| (x,)))?;

        color_matrix = match self.kind {
            ColorMatrixType::Matrix(matrix) => {
                color_matrix
                    .attr("type", "matrix")?
                    .write_attr("values", |out| {
                        write_spaced(out, matrix.iter().flatten(), |out, value| {
                            out.write_float(value.get())
                        })
                    })
            }
            ColorMatrixType::Saturate(value) => {
                color_matrix.attr("type", "saturate")?.attr("values", value)
            }
            ColorMatrixType::HueRotate(value) => color_matrix
                .attr("type", "hueRotate")?
                .attr("values", value),
            ColorMatrixType::LuminanceToAlpha => color_matrix.attr("type", "luminanceToAlpha"),
        }?;

        color_matrix
            .attr_if(
                "color-interpolation-filters",
                (&self.color_interpolation,),
                self.color_interpolation != ColorInterpolation::LinearRgb,
            )?
            .attr("result", (self.iri(),))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, ColorMatrix> {
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input = Some(input.into());
        self
    }

    pub fn matrix(mut self, matrix: [[f32; 5]; 4]) -> Self {
        self.inner.kind = ColorMatrixType::matrix(matrix);
        self
    }

    pub fn saturate(mut self, amount: f32) -> Self {
        self.inner.kind = ColorMatrixType::Saturate(pf32!(amount));
        self
    }

    pub fn hue_rotate(mut self, angle: f32) -> Self {
        self.inner.kind = ColorMatrixType::HueRotate(ff32!(angle));
        self
    }

    pub fn luminance_to_alpha(mut self) -> Self {
        self.inner.kind = ColorMatrixType::LuminanceToAlpha;
        self
    }

    pub fn identity(mut self) -> Self {
        self.inner.kind = ColorMatrixType::identity();
        self
    }

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
        ctx.color_matrix()
            .luminance_to_alpha()
            .x(0.5)
            .y(0.6)
            .width(110)
            .height(120)
            .finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feColorMatrix type="luminanceToAlpha" x="0.5" y="0.6" width="110" height="120" result="{}" />"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders() {
        let ctx = FilterContext::default();
        ctx.color_matrix().finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feColorMatrix type="matrix" values="1 0 0 0 0 0 1 0 0 0 0 0 1 0 0 0 0 0 1 0" result="{}" />"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_with_explicit_matrix() {
        let ctx = FilterContext::default();

        ctx.color_matrix()
            .matrix([
                [1.0, 0.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 0.0, 2.0],
                [0.0, 0.0, 1.0, 0.0, 3.0],
                [0.0, 0.0, 0.0, 1.0, 4.0],
            ])
            .finish();

        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feColorMatrix type="matrix" values="1 0 0 0 1 0 1 0 0 2 0 0 1 0 3 0 0 0 1 4" result="{}" />"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_saturate_type() {
        let ctx = FilterContext::default();
        ctx.color_matrix().saturate(0.5).finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feColorMatrix type="saturate" values="0.5" result="{}" />"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_hue_rotate_type() {
        let ctx = FilterContext::default();
        ctx.color_matrix().hue_rotate(30.0).finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feColorMatrix type="hueRotate" values="30" result="{}" />"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_luminance_to_alpha_type() {
        let ctx = FilterContext::default();
        ctx.color_matrix().luminance_to_alpha().finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"<feColorMatrix type="luminanceToAlpha" result="{}" />"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders_with_attrs() {
        let ctx = FilterContext::default();
        let input = FilterInput::source_graphic();
        let color_interpolation = ColorInterpolation::SRgb;

        ctx.color_matrix()
            .luminance_to_alpha()
            .input(input)
            .color_interpolation(color_interpolation)
            .finish();

        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feColorMatrix
    type="luminanceToAlpha"
    in="{input}"
    color-interpolation-filters="{color_interpolation}"
    result="{}"
/>
"#,
                node.iri()
            ),
        );
    }
}
