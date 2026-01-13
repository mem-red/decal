use crate::attributes::IntoPaint;
use crate::macros::nf32;
use crate::paint::{IntoResources, Resource, ResourceIri};
use crate::primitives::{BlendMode, Color, PatternUnits};
use crate::primitives::{LinearGradient, Pattern, RadialGradient};
use crate::utils::{ElementWriter, IsDefault};
use quick_xml::escape::escape;
use std::fmt::Display;
use strict_num::NormalizedF32;

#[derive(Debug, Clone, Default)]
pub enum Paint {
    #[default]
    None,
    Color(Color),
    Image(Pattern),
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

    // TODO add image position and other properties (maybe an image paint builder)
    pub fn image(href: &str, width: f32, height: f32) -> Self {
        let pattern = match Pattern::build(|out| {
            ElementWriter::new(out, "image")?
                .attr("href", escape(href).as_ref())?
                .attrs([("width", width), ("height", height)])?
                .close()
        }) {
            Ok(builder) => builder.pattern_units(PatternUnits::ObjectBoundingBox),
            _ => return Self::none(),
        };

        Self::Image(pattern)
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

    pub(crate) fn is_none(&self) -> bool {
        matches!(self, Paint::None)
    }
}

impl Display for Paint {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Paint::None => f.write_str("none"),
            Paint::Color(color) => color.fmt(f),
            Paint::LinearGradient(gradient) => write!(f, "url(#{})", gradient.iri()),
            Paint::RadialGradient(gradient) => write!(f, "url(#{})", gradient.iri()),
            Paint::Image(pattern) | Paint::Pattern(pattern) => write!(f, "url(#{})", pattern.iri()),
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

//

#[derive(Debug, Clone)]
pub struct PaintLayer {
    pub(crate) paint: Paint,
    pub(crate) blend_mode: BlendMode,
    pub(crate) opacity: NormalizedF32,
}

impl Default for PaintLayer {
    fn default() -> Self {
        Self {
            paint: Default::default(),
            blend_mode: Default::default(),
            opacity: NormalizedF32::ONE,
        }
    }
}

impl PaintLayer {
    pub fn blend_mode(mut self, blend_mode: BlendMode) -> Self {
        self.blend_mode = blend_mode;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = nf32!(opacity);
        self
    }

    pub(crate) fn is_none(&self) -> bool {
        self.paint.is_none()
    }
}

impl<T> From<T> for PaintLayer
where
    T: IntoPaint,
{
    #[inline]
    fn from(value: T) -> Self {
        Self {
            paint: value.into_paint(),
            ..Default::default()
        }
    }
}

//

#[derive(Debug, Clone, Default)]
pub struct PaintStack(Vec<PaintLayer>);

impl PaintStack {
    pub(crate) fn new<I, T>(layers: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<PaintLayer>,
    {
        Self(layers.into_iter().map(Into::into).collect())
    }

    pub(crate) fn is_none(&self) -> bool {
        self.0.is_empty() || (self.0.len() == 1 && self.0[0].is_none())
    }

    pub(crate) fn needs_isolation(&self) -> bool {
        self.0.iter().any(|x| !x.blend_mode.is_default())
    }

    pub(crate) fn layers(&self) -> &[PaintLayer] {
        &self.0
    }
}

impl IntoResources for PaintStack {
    fn into_resources(self) -> Vec<Resource> {
        self.0
            .into_iter()
            .map(|x| x.paint.into_resources())
            .flatten()
            .collect()
    }
}
