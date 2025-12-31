use crate::filters::primitives::PrimitiveBuilder;
use crate::macros::{ff32, pf32};
use crate::paint::ResourceIri;
use crate::primitives::FilterInput;
use crate::utils::FloatWriter;
use crate::utils::IsDefault;
use enum_display::EnumDisplay;
use std::fmt::{Display, Formatter, Write};
use strict_num::{FiniteF32, PositiveF32};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, EnumDisplay)]
pub enum ColorMatrixType {
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
    fn matrix<T>(matrix: [[T; 5]; 4]) -> Self
    where
        T: Into<f32> + Copy,
    {
        let mut ff32_matrix: [[FiniteF32; 5]; 4] = [[FiniteF32::default(); 5]; 4];

        for (i, row) in matrix.iter().enumerate() {
            for (j, element) in row.iter().enumerate() {
                ff32_matrix[i][j] = ff32!(*element);
            }
        }

        Self::Matrix(ff32_matrix)
    }

    pub fn identity() -> Self {
        Self::matrix([
            [1.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0, 0.0],
        ])
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
pub struct ColorMatrix {
    input: Option<FilterInput>,
    kind: ColorMatrixType,
}

impl ColorMatrix {
    pub(crate) fn new() -> Self {
        ColorMatrix::default()
    }
}

impl ResourceIri for ColorMatrix {}

impl Display for ColorMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"<feColorMatrix"#)?;

        match self.kind {
            ColorMatrixType::Matrix(matrix) => {
                write!(f, r#" type="matrix" values=""#)?;
                let mut first = true;

                for row in matrix {
                    for i in row {
                        if !first {
                            f.write_char(' ')?;
                        }

                        f.write_float(i.get())?;
                        first = false;
                    }
                }

                write!(f, r#"""#)?;
            }
            ColorMatrixType::Saturate(value) => {
                write!(f, r#" type="saturate" values=""#)?;
                f.write_float(value.get())?;
                write!(f, r#"""#)?;
            }
            ColorMatrixType::HueRotate(value) => {
                write!(f, r#" type="hueRotate" values=""#)?;
                f.write_float(value.get())?;
                write!(f, r#"""#)?;
            }
            ColorMatrixType::LuminanceToAlpha => {
                write!(f, r#" type="luminanceToAlpha""#)?;
            }
        }

        if let Some(input) = self.input {
            write!(f, r#" in="{input}""#)?;
        }

        write!(f, r#" result="{}" />"#, self.iri())
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

    pub fn matrix<T>(mut self, matrix: [[T; 5]; 4]) -> Self
    where
        T: Into<f32> + Copy,
    {
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
}
