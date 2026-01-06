use crate::macros::ff32;
use crate::utils::ElementWriter;
use std::fmt::{Display, Formatter};
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
        match *self {
            Self::DistantLight { azimuth, elevation } => ElementWriter::new(f, "feDistantLight")?
                .attrs([("azimuth", azimuth), ("elevation", elevation)])?
                .close(),
            Self::PointLight { x, y, z } => ElementWriter::new(f, "fePointLight")?
                .attrs([("x", x), ("y", y), ("z", z)])?
                .close(),
            Self::SpotLight {
                x,
                y,
                z,
                points_at_x,
                points_at_y,
                points_at_z,
                specular_exponent,
                limiting_cone_angle,
            } => ElementWriter::new(f, "feSpotLight")?
                .attrs([
                    ("x", x),
                    ("y", y),
                    ("z", z),
                    ("pointsAtX", points_at_x),
                    ("pointsAtY", points_at_y),
                    ("pointsAtZ", points_at_z),
                ])?
                .attr_if(
                    "specularExponent",
                    specular_exponent,
                    specular_exponent.get() != 1.0,
                )?
                .attr("limitingConeAngle", limiting_cone_angle)?
                .close(),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct LightSource(LightSourceInner);

impl LightSource {
    pub fn distant_light(azimuth: f32, elevation: f32) -> Self {
        Self(LightSourceInner::DistantLight {
            azimuth: ff32!(azimuth),
            elevation: ff32!(elevation),
        })
    }

    pub fn point_light(x: f32, y: f32, z: f32) -> Self {
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
