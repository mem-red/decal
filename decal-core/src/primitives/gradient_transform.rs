use crate::macros::ff32;
use crate::primitives::IntoFloatPair;
use crate::utils::FloatWriter;
use crate::utils::IsDefault;
use std::fmt::Write;
use strict_num::FiniteF32;
use usvg::Transform;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct GradientTransform {
    sx: FiniteF32,
    kx: FiniteF32,
    ky: FiniteF32,
    sy: FiniteF32,
    tx: FiniteF32,
    ty: FiniteF32,
}

impl Default for GradientTransform {
    fn default() -> Self {
        GradientTransform {
            sx: ff32!(1.0),
            kx: ff32!(0.0),
            ky: ff32!(0.0),
            sy: ff32!(1.0),
            tx: ff32!(0.0),
            ty: ff32!(0.0),
        }
    }
}

impl From<Transform> for GradientTransform {
    fn from(value: Transform) -> Self {
        GradientTransform {
            sx: ff32!(value.sx, 1.0),
            kx: ff32!(value.kx),
            ky: ff32!(value.ky),
            sy: ff32!(value.sy, 1.0),
            tx: ff32!(value.tx),
            ty: ff32!(value.ty),
        }
    }
}

impl IsDefault for GradientTransform {}

impl GradientTransform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_identity(&self) -> bool {
        self.is_default()
    }

    //

    pub fn translate<T>(self, value: T) -> Self
    where
        T: IntoFloatPair,
    {
        let (x, y) = value.into_float_pair();
        self.transform().post_translate(x, y).into()
    }

    pub fn translate_x<T>(self, value: T) -> Self
    where
        T: Into<f32>,
    {
        self.transform().post_translate(value.into(), 0.0).into()
    }

    pub fn translate_y<T>(self, value: T) -> Self
    where
        T: Into<f32>,
    {
        self.transform().post_translate(0.0, value.into()).into()
    }

    //

    pub fn scale<T>(self, value: T) -> Self
    where
        T: IntoFloatPair,
    {
        let (x, y) = value.into_float_pair();
        self.transform().post_scale(x, y).into()
    }

    pub fn scale_x<T>(self, value: T) -> Self
    where
        T: Into<f32>,
    {
        self.transform().post_scale(value.into(), 1.0).into()
    }

    pub fn scale_y<T>(self, value: T) -> Self
    where
        T: Into<f32>,
    {
        self.transform().post_scale(1.0, value.into()).into()
    }

    //

    pub fn rotate<T>(self, angle: T) -> Self
    where
        T: Into<f32>,
    {
        self.transform().post_rotate(angle.into()).into()
    }

    pub fn rotate_at<T>(self, angle: T, x: T, y: T) -> Self
    where
        T: Into<f32>,
    {
        self.transform()
            .post_rotate_at(angle.into(), x.into(), y.into())
            .into()
    }

    //

    pub fn skew<T>(self, value: T) -> Self
    where
        T: IntoFloatPair,
    {
        let (x, y) = value.into_float_pair();
        self.transform()
            .post_concat(Transform::from_skew(x, y))
            .into()
    }

    pub fn skew_x(self, angle: f32) -> Self {
        self.transform()
            .post_concat(Transform::from_skew(angle, 0.0))
            .into()
    }

    pub fn skew_y(self, angle: f32) -> Self {
        self.transform()
            .post_concat(Transform::from_skew(0.0, angle))
            .into()
    }

    //

    pub(crate) fn write<T>(&self, out: &mut T) -> std::fmt::Result
    where
        T: Write,
    {
        let tf = self.transform();

        if tf.is_identity() || !tf.is_valid() {
            return Ok(());
        }

        out.write_str(r#" gradientTransform="matrix("#)?;
        out.write_float(tf.sx)?;
        out.write_char(' ')?;
        out.write_float(tf.ky)?;
        out.write_char(' ')?;
        out.write_float(tf.kx)?;
        out.write_char(' ')?;
        out.write_float(tf.sy)?;
        out.write_char(' ')?;
        out.write_float(tf.tx)?;
        out.write_char(' ')?;
        out.write_float(tf.ty)?;
        out.write_str(r#")""#)
    }

    //

    fn transform(&self) -> Transform {
        Transform {
            sx: self.sx.get(),
            kx: self.kx.get(),
            ky: self.ky.get(),
            sy: self.sy.get(),
            tx: self.tx.get(),
            ty: self.ty.get(),
        }
    }
}
