use crate::utils::FloatWriter;
use std::fmt::Write;
use usvg::tiny_skia_path;

#[derive(Debug, Clone)]
enum RotationAnchor {
    Center,          // rotate around object's center
    Origin,          // rotate around origin of system coords
    Point(f32, f32), // rotate around a point
}

#[derive(Debug, Clone)]
enum TransformOperation {
    Translate(f32, f32),
    Scale(f32, f32),
    Rotate(f32, RotationAnchor),
    Skew(f32, f32),
}

#[derive(Debug, Clone, Default)]
pub struct Transform(Vec<TransformOperation>);

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }

    //

    pub fn translate<T>(mut self, value: T) -> Self
    where
        T: IntoFloatPair,
    {
        let (x, y) = value.into_float_pair();
        self.0.push(TransformOperation::Translate(x, y));
        self
    }

    pub fn translate_x<T>(mut self, value: T) -> Self
    where
        T: Into<f32>,
    {
        self.0
            .push(TransformOperation::Translate(value.into(), 0.0));
        self
    }

    pub fn translate_y<T>(mut self, value: T) -> Self
    where
        T: Into<f32>,
    {
        self.0
            .push(TransformOperation::Translate(0.0, value.into()));
        self
    }

    //

    pub fn scale<T>(mut self, value: T) -> Self
    where
        T: IntoFloatPair,
    {
        let (x, y) = value.into_float_pair();
        self.0.push(TransformOperation::Scale(x, y));
        self
    }

    pub fn scale_x<T>(mut self, value: T) -> Self
    where
        T: Into<f32>,
    {
        self.0.push(TransformOperation::Scale(value.into(), 1.0));
        self
    }

    pub fn scale_y<T>(mut self, value: T) -> Self
    where
        T: Into<f32>,
    {
        self.0.push(TransformOperation::Scale(1.0, value.into()));
        self
    }

    //

    pub fn rotate<T>(mut self, angle: T) -> Self
    where
        T: Into<f32>,
    {
        self.0.push(TransformOperation::Rotate(
            angle.into(),
            RotationAnchor::Center,
        ));
        self
    }

    pub fn rotate_origin<T>(mut self, angle: T) -> Self
    where
        T: Into<f32>,
    {
        self.0.push(TransformOperation::Rotate(
            angle.into(),
            RotationAnchor::Origin,
        ));
        self
    }

    pub fn rotate_at<T>(mut self, angle: T, x: T, y: T) -> Self
    where
        T: Into<f32>,
    {
        self.0.push(TransformOperation::Rotate(
            angle.into(),
            RotationAnchor::Point(x.into(), y.into()),
        ));
        self
    }

    //

    pub fn skew<T>(mut self, value: T) -> Self
    where
        T: IntoFloatPair,
    {
        let (x, y) = value.into_float_pair();
        self.0.push(TransformOperation::Skew(x, y));
        self
    }

    pub fn skew_x<T>(mut self, angle: T) -> Self
    where
        T: Into<f32>,
    {
        self.0.push(TransformOperation::Skew(angle.into(), 0.0));
        self
    }

    pub fn skew_y<T>(mut self, angle: T) -> Self
    where
        T: Into<f32>,
    {
        self.0.push(TransformOperation::Skew(0.0, angle.into()));
        self
    }

    //

    pub(crate) fn write<T>(
        &self,
        out: &mut T,
        pos: (f32, f32),
        translate: (f32, f32),
        size: (f32, f32),
    ) -> std::fmt::Result
    where
        T: Write,
    {
        let mut tf = usvg::Transform::from_translate(translate.0, translate.1);
        let center = tiny_skia_path::Point {
            x: pos.0 + size.0 / 2.0,
            y: pos.1 + size.1 / 2.0,
        };

        for op in &self.0 {
            // center of obj before ops
            let mut before = center;
            tf.map_point(&mut before);
            let mut recenter = true;

            match *op {
                TransformOperation::Translate(x, y) => {
                    tf = tf.post_translate(x, y);
                    recenter = false;
                }

                TransformOperation::Scale(x, y) => {
                    tf = tf.post_scale(x, y);
                }

                TransformOperation::Rotate(angle, ref anchor) => match anchor {
                    RotationAnchor::Center => {
                        tf = tf.post_rotate(angle);
                    }
                    RotationAnchor::Origin => {
                        tf = tf.post_rotate(angle);
                        recenter = false;
                    }
                    RotationAnchor::Point(x, y) => {
                        tf = tf.post_rotate_at(angle, *x, *y);
                        recenter = false;
                    }
                },

                TransformOperation::Skew(x, y) => {
                    tf = tf.post_concat(usvg::Transform::from_skew(x, y));
                }
            }

            if recenter {
                let mut after = center;
                // center of obj after ops
                tf.map_point(&mut after);
                tf = tf.post_translate(before.x - after.x, before.y - after.y);
            }
        }

        if tf.is_identity() || !tf.is_valid() {
            return Ok(());
        }

        write!(out, r#" transform="matrix("#)?;
        FloatWriter::write_float(out, tf.sx)?;
        out.write_char(' ')?;
        FloatWriter::write_float(out, tf.ky)?;
        out.write_char(' ')?;
        FloatWriter::write_float(out, tf.kx)?;
        out.write_char(' ')?;
        FloatWriter::write_float(out, tf.sy)?;
        out.write_char(' ')?;
        FloatWriter::write_float(out, tf.tx)?;
        out.write_char(' ')?;
        FloatWriter::write_float(out, tf.ty)?;
        out.write_str(r#")""#)
    }
}

pub trait IntoFloatPair {
    fn into_float_pair(self) -> (f32, f32);
}

impl IntoFloatPair for f32 {
    fn into_float_pair(self) -> (f32, f32) {
        (self, self)
    }
}

impl<T> IntoFloatPair for (T, T)
where
    T: Into<f32>,
{
    fn into_float_pair(self) -> (f32, f32) {
        (self.0.into(), self.1.into())
    }
}

impl<T> IntoFloatPair for [T; 1]
where
    T: Into<f32> + Copy,
{
    fn into_float_pair(self) -> (f32, f32) {
        (self[0].into(), self[0].into())
    }
}

impl<T> IntoFloatPair for [T; 2]
where
    T: Into<f32> + Copy,
{
    fn into_float_pair(self) -> (f32, f32) {
        (self[0].into(), self[1].into())
    }
}
