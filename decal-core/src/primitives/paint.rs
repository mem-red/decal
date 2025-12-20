use crate::prelude::{LinearGradient, Pattern, RadialGradient};
use crate::primitives::{Color, ResourceIri};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default)]
pub enum Paint {
    #[default]
    None,
    Color(Color),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Pattern(Pattern),
}

impl Paint {
    pub const fn none() -> Self {
        Self::None
    }

    pub const fn color(color: Color) -> Self {
        Self::Color(color)
    }

    pub const fn linear_gradient(linear_gradient: LinearGradient) -> Self {
        Self::LinearGradient(linear_gradient)
    }

    pub const fn radial_gradient(radial_gradient: RadialGradient) -> Self {
        Self::RadialGradient(radial_gradient)
    }

    pub const fn pattern(pattern: Pattern) -> Self {
        Self::Pattern(pattern)
    }
}

impl Display for Paint {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Paint::None => write!(f, "none"),
            Paint::Color(color) => write!(f, "{color}"),
            Paint::LinearGradient(gradient) => write!(f, "url(#{})", gradient.iri()),
            Paint::RadialGradient(gradient) => write!(f, "url(#{})", gradient.iri()),
            _ => todo!(),
        }
    }
}

impl From<Color> for Paint {
    #[inline]
    fn from(value: Color) -> Self {
        Paint::Color(value)
    }
}

impl From<LinearGradient> for Paint {
    #[inline]
    fn from(value: LinearGradient) -> Self {
        Paint::LinearGradient(value)
    }
}

impl From<RadialGradient> for Paint {
    #[inline]
    fn from(value: RadialGradient) -> Self {
        Paint::RadialGradient(value)
    }
}

impl From<Pattern> for Paint {
    #[inline]
    fn from(value: Pattern) -> Self {
        Paint::Pattern(value)
    }
}
