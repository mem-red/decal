use crate::macros::ff32;
use crate::utils::FloatWriter;
use std::fmt::{Display, Formatter, Write};
use strict_num::FiniteF32;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum LightSourceInner {
    DistantLight {
        azimuth: FiniteF32,
        elevation: FiniteF32,
    },
    PointLight {
        x: FiniteF32,
        y: FiniteF32,
        z: FiniteF32,
    },
    SpotLight {
        x: FiniteF32,
        y: FiniteF32,
        z: FiniteF32,
        points_at_x: FiniteF32,
        points_at_y: FiniteF32,
        points_at_z: FiniteF32,
        specular_exponent: FiniteF32,
        limiting_cone_angle: Option<FiniteF32>,
    },
}

impl Display for LightSourceInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DistantLight { azimuth, elevation } => {
                f.write_str(r#"<feDistantLight azimuth=""#)?;
                f.write_float(azimuth.get())?;
                f.write_str(r#"" elevation=""#)?;
                f.write_float(elevation.get())?;
                f.write_str(r#"" />"#)
            }
            Self::PointLight { x, y, z } => {
                f.write_str(r#"<fePointLight x=""#)?;
                f.write_float(x.get())?;
                f.write_str(r#"" y=""#)?;
                f.write_float(y.get())?;
                f.write_str(r#"" z=""#)?;
                f.write_float(z.get())?;
                f.write_str(r#"" />"#)
            }
            Self::SpotLight {
                x,
                y,
                z,
                points_at_x,
                points_at_y,
                points_at_z,
                specular_exponent,
                limiting_cone_angle,
            } => {
                f.write_str(r#"<feSpotLight x=""#)?;
                f.write_float(x.get())?;
                f.write_str(r#"" y=""#)?;
                f.write_float(y.get())?;
                f.write_str(r#"" z=""#)?;
                f.write_float(z.get())?;
                f.write_str(r#"" pointsAtX=""#)?;
                f.write_float(points_at_x.get())?;
                f.write_str(r#"" pointsAtY=""#)?;
                f.write_float(points_at_y.get())?;
                f.write_str(r#"" pointsAtZ=""#)?;
                f.write_float(points_at_z.get())?;
                f.write_char('"')?;

                if specular_exponent.get() != 1.0 {
                    f.write_str(r#" specularExponent=""#)?;
                    f.write_float(specular_exponent.get())?;
                    f.write_char('"')?;
                }

                if let Some(angle) = limiting_cone_angle {
                    f.write_str(r#" limitingConeAngle=""#)?;
                    f.write_float(angle.get())?;
                    f.write_char('"')?;
                }

                f.write_str(r#" />"#)
            }
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct LightSource(LightSourceInner);

impl LightSource {
    pub fn distant_light<T>(azimuth: T, elevation: T) -> Self
    where
        T: Into<f32>,
    {
        Self(LightSourceInner::DistantLight {
            azimuth: ff32!(azimuth),
            elevation: ff32!(elevation),
        })
    }

    pub fn point_light<T>(x: T, y: T, z: T) -> Self
    where
        T: Into<f32>,
    {
        Self(LightSourceInner::PointLight {
            x: ff32!(x),
            y: ff32!(y),
            z: ff32!(z),
        })
    }

    pub fn spot_light(
        location: (f32, f32, f32),
        points_at: (f32, f32, f32),
        specular_exponent: Option<f32>,
        limiting_cone_angle: Option<f32>,
    ) -> Self {
        Self(LightSourceInner::SpotLight {
            x: ff32!(location.0),
            y: ff32!(location.1),
            z: ff32!(location.2),
            points_at_x: ff32!(points_at.0),
            points_at_y: ff32!(points_at.1),
            points_at_z: ff32!(points_at.2),
            specular_exponent: specular_exponent.map(|x| ff32!(x)).unwrap_or(ff32!(1.0)),
            limiting_cone_angle: limiting_cone_angle.map(|x| ff32!(x)),
        })
    }
}

impl Display for LightSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
