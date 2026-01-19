use crate::{
    macros::ff32,
    primitives::IntoFloatPair,
    utils::{
        FloatWriter,
        IsDefault,
    },
};
use smart_default::SmartDefault;
use std::fmt::Write;
use strict_num::FiniteF32;
use usvg::Transform;

pub type GradientTransform = PaintTransform;
pub type PatternTransform = PaintTransform;

#[derive(Debug, Hash, Eq, PartialEq, Clone, SmartDefault)]
pub struct PaintTransform {
    #[default(ff32!(1.0))]
    sx: FiniteF32,
    kx: FiniteF32,
    ky: FiniteF32,
    #[default(ff32!(1.0))]
    sy: FiniteF32,
    tx: FiniteF32,
    ty: FiniteF32,
}

impl From<Transform> for PaintTransform {
    fn from(value: Transform) -> Self {
        PaintTransform {
            sx: ff32!(value.sx, 1.0),
            kx: ff32!(value.kx),
            ky: ff32!(value.ky),
            sy: ff32!(value.sy, 1.0),
            tx: ff32!(value.tx),
            ty: ff32!(value.ty),
        }
    }
}

impl IsDefault for PaintTransform {}

impl PaintTransform {
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

    pub fn translate_x(self, value: f32) -> Self {
        self.transform().post_translate(value, 0.0).into()
    }

    pub fn translate_y(self, value: f32) -> Self {
        self.transform().post_translate(0.0, value).into()
    }

    //

    pub fn scale<T>(self, value: T) -> Self
    where
        T: IntoFloatPair,
    {
        let (x, y) = value.into_float_pair();
        self.transform().post_scale(x, y).into()
    }

    pub fn scale_x(self, value: f32) -> Self {
        self.transform().post_scale(value, 1.0).into()
    }

    pub fn scale_y(self, value: f32) -> Self {
        self.transform().post_scale(1.0, value).into()
    }

    //

    pub fn rotate(self, angle: f32) -> Self {
        self.transform().post_rotate(angle).into()
    }

    pub fn rotate_at(self, angle: f32, x: f32, y: f32) -> Self {
        self.transform().post_rotate_at(angle, x, y).into()
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

    pub(crate) fn write<T>(&self, out: &mut T, attr_name: &str) -> std::fmt::Result
    where
        T: Write,
    {
        let tf = self.transform();

        if tf.is_identity() || !tf.is_valid() {
            return Ok(());
        }

        out.write_char(' ')?;
        out.write_str(attr_name)?;
        out.write_str(r#"="matrix("#)?;
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
