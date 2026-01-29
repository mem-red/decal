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

/// The gradient transformation.
pub type GradientTransform = PaintTransform;

/// The pattern transformation.
pub type PatternTransform = PaintTransform;

/// The 2D transformation.
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
    /// Creates a new identity [`PaintTransform`] instance.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if the transform represents the identity transformation.
    pub fn is_identity(&self) -> bool {
        self.is_default()
    }

    /// Translates the paint along both axes.
    ///
    /// # Arguments
    /// - `value`: The translation distance convertible using [`IntoFloatPair`].
    ///
    /// # Returns
    /// - [`Self`]
    pub fn translate<T>(self, value: T) -> Self
    where
        T: IntoFloatPair,
    {
        let (x, y) = value.into_float_pair();
        self.transform().post_translate(x, y).into()
    }

    /// Translates the paint along the x-axis.
    ///
    /// # Arguments
    /// - `value`: The translation distance along the x-axis.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn translate_x(self, value: f32) -> Self {
        self.transform().post_translate(value, 0.0).into()
    }

    /// Translates the paint along the y-axis.
    ///
    /// # Arguments
    /// - `value`: The translation distance along the y-axis.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn translate_y(self, value: f32) -> Self {
        self.transform().post_translate(0.0, value).into()
    }

    /// Scales the paint along both axes.
    ///
    /// # Arguments
    /// - `value`: The scaling factor convertible using [`IntoFloatPair`].
    ///
    /// # Returns
    /// - [`Self`]
    pub fn scale<T>(self, value: T) -> Self
    where
        T: IntoFloatPair,
    {
        let (x, y) = value.into_float_pair();
        self.transform().post_scale(x, y).into()
    }

    /// Scales the paint along the x-axis.
    ///
    /// # Arguments
    /// - `value`: The scaling factor along the x-axis.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn scale_x(self, value: f32) -> Self {
        self.transform().post_scale(value, 1.0).into()
    }

    /// Scales the paint along the y-axis.
    ///
    /// # Arguments
    /// - `value`: The scaling factor along the y-axis.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn scale_y(self, value: f32) -> Self {
        self.transform().post_scale(1.0, value).into()
    }

    /// Rotates the paint around the coordinate origin.
    ///
    /// # Arguments
    /// - `angle`: The rotation angle in degrees.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn rotate(self, angle: f32) -> Self {
        self.transform().post_rotate(angle).into()
    }

    /// Rotates the paint around a specific point.
    ///
    /// # Arguments
    /// - `angle`: The rotation angle in degrees.
    /// - `x`: The x coordinate of the rotation anchor.
    /// - `y`: The y coordinate of the rotation anchor.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn rotate_at(self, angle: f32, x: f32, y: f32) -> Self {
        self.transform().post_rotate_at(angle, x, y).into()
    }

    /// Skews the paint along both axes.
    ///
    /// # Arguments
    /// - `value`: The skew angle convertible using [`IntoFloatPair`].
    ///
    /// # Returns
    /// - [`Self`]
    pub fn skew<T>(self, value: T) -> Self
    where
        T: IntoFloatPair,
    {
        let (x, y) = value.into_float_pair();
        self.transform()
            .post_concat(Transform::from_skew(x, y))
            .into()
    }

    /// Skews the paint along the x-axis.
    ///
    /// # Arguments
    /// - `value`: The skew angle along the x-axis.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn skew_x(self, angle: f32) -> Self {
        self.transform()
            .post_concat(Transform::from_skew(angle, 0.0))
            .into()
    }

    /// Skews the paint along the y-axis.
    ///
    /// # Arguments
    /// - `value`: The skew angle along the y-axis.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn skew_y(self, angle: f32) -> Self {
        self.transform()
            .post_concat(Transform::from_skew(0.0, angle))
            .into()
    }

    /// Writes the resolved transform as an SVG matrix attribute.
    ///
    /// # Arguments
    /// - `out`: The output writer.
    /// - `attr_name`: The name of the transform attribute.
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

    /// Converts the current [`PaintTransform`] into [`tiny_skia::Transform`].
    ///
    /// # Returns
    /// - [`tiny_skia::Transform`]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::str_sink;

    fn transform_sink(tf: PaintTransform) -> String {
        str_sink(|out| tf.write(out, "key"))
    }

    #[test]
    fn defaults_to_identity() {
        let tf = PaintTransform::new();
        assert!(tf.is_identity());
        assert!(tf.is_default());
        assert!(transform_sink(tf).is_empty());
    }

    //

    #[test]
    fn translates_xy() {
        let tf = PaintTransform::new().translate((1.0, 2.0)).transform();
        assert_eq!(tf.tx, 1.0);
        assert_eq!(tf.ty, 2.0);
    }

    #[test]
    fn translates_x() {
        let tf = PaintTransform::new().translate_x(1.0).transform();
        assert_eq!(tf.tx, 1.0);
        assert_eq!(tf.ty, 0.0);
    }

    #[test]
    fn translates_y() {
        let tf = PaintTransform::new().translate_y(1.0).transform();
        assert_eq!(tf.ty, 1.0);
        assert_eq!(tf.tx, 0.0);
    }

    //

    #[test]
    fn scales_uniformly() {
        let tf = PaintTransform::new().scale(2.0).transform();
        assert_eq!(tf.sx, 2.0);
        assert_eq!(tf.sy, 2.0);
    }

    #[test]
    fn scales_xy() {
        let tf = PaintTransform::new().scale((2.0, 2.5)).transform();
        assert_eq!(tf.sx, 2.0);
        assert_eq!(tf.sy, 2.5);
    }

    #[test]
    fn scales_x() {
        let tf = PaintTransform::new().scale_x(2.0).transform();
        assert_eq!(tf.sx, 2.0);
        assert_eq!(tf.sy, 1.0);
    }

    #[test]
    fn scales_y() {
        let tf = PaintTransform::new().scale_y(2.0).transform();
        assert_eq!(tf.sy, 2.0);
        assert_eq!(tf.sx, 1.0);
    }

    //

    #[test]
    fn skews_uniformly() {
        let tf = PaintTransform::new().skew(2.0).transform();
        assert_eq!(tf.kx, 2.0);
        assert_eq!(tf.ky, 2.0);
    }

    #[test]
    fn skews_xy() {
        let tf = PaintTransform::new().skew((2.0, 2.5)).transform();
        assert_eq!(tf.kx, 2.0);
        assert_eq!(tf.ky, 2.5);
    }

    #[test]
    fn skews_x() {
        let tf = PaintTransform::new().skew_x(1.0).transform();
        assert_eq!(tf.kx, 1.0);
        assert_eq!(tf.ky, 0.0);
    }

    #[test]
    fn skews_y() {
        let tf = PaintTransform::new().skew_y(1.0).transform();
        assert_eq!(tf.ky, 1.0);
        assert_eq!(tf.kx, 0.0);
    }

    //

    #[test]
    fn does_not_render_identity_matrix() {
        assert!(transform_sink(PaintTransform::default()).is_empty());
    }

    #[test]
    fn renders_matrix() {
        assert_eq!(
            transform_sink(PaintTransform::new().scale(2.5).translate((10.0, 5.0))),
            r#" key="matrix(2.5 0 0 2.5 10 5)""#
        );
    }
}
