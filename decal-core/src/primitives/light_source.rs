use crate::{
    macros::ff32,
    utils::ElementWriter,
};
use std::fmt::{
    Display,
    Formatter,
};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_distant_light() {
        assert_eq!(
            LightSource::distant_light(45.0, 60.0).to_string(),
            r#"<feDistantLight azimuth="45" elevation="60" />"#
        );
    }

    #[test]
    fn renders_point_light() {
        assert_eq!(
            LightSource::point_light(1.0, 2.0, 3.0).to_string(),
            r#"<fePointLight x="1" y="2" z="3" />"#
        );
    }

    #[test]
    fn renders_spot_light() {
        assert_eq!(
            LightSource::spot_light((1.0, 2.0, 3.0), (4.0, 5.0, 6.0), None, None).to_string(),
            r#"<feSpotLight x="1" y="2" z="3" pointsAtX="4" pointsAtY="5" pointsAtZ="6" />"#
        );

        assert_eq!(
            LightSource::spot_light((1.0, 2.0, 3.0), (4.0, 5.0, 6.0), Some(1.5), Some(45.0))
                .to_string(),
            r#"<feSpotLight x="1" y="2" z="3" pointsAtX="4" pointsAtY="5" pointsAtZ="6" specularExponent="1.5" limitingConeAngle="45" />"#
        );
    }
}
