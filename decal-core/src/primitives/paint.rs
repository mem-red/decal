use crate::{
    layout::{
        ImageSource,
        RenderContext,
    },
    macros::nf32,
    paint::{
        IntoResources,
        Resource,
        ResourceIri,
    },
    primitives::{
        BlendMode,
        Color,
        CrossOrigin,
        Length,
        LinearGradient,
        Path,
        Pattern,
        PatternContentUnits,
        PatternUnits,
        RadialGradient,
    },
    utils::{
        ElementWriter,
        Initialized,
        IsDefault,
    },
};
use smart_default::SmartDefault;
use std::fmt::{
    Display,
    Write,
};
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

    pub fn image(image: ImagePaint) -> Self {
        image
            .into_pattern()
            .map(Self::Image)
            .unwrap_or(Self::none())
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

//

#[derive(Debug, Clone, SmartDefault)]
pub struct ImagePaint {
    source: ImageSource,
    #[default(nf32!(0.5))]
    x: NormalizedF32,
    #[default(nf32!(0.5))]
    y: NormalizedF32,
    #[default(NormalizedF32::ONE)]
    width: NormalizedF32,
    #[default(NormalizedF32::ONE)]
    height: NormalizedF32,
    cross_origin: Option<CrossOrigin>,
}

impl ImagePaint {
    pub fn new<S>(source: S) -> Self
    where
        S: Into<ImageSource>,
    {
        Self {
            source: source.into(),
            ..Default::default()
        }
    }

    pub fn x(mut self, x: f32) -> Self {
        self.x = nf32!(x);
        self
    }

    pub fn y(mut self, y: f32) -> Self {
        self.y = nf32!(y);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = nf32!(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = nf32!(height);
        self
    }

    pub fn cross_origin<T>(mut self, value: T) -> Self
    where
        T: Into<Option<CrossOrigin>>,
    {
        self.cross_origin = value.into();
        self
    }

    //

    pub fn top_left(mut self) -> Self {
        self.x = nf32!(0.0);
        self.y = nf32!(0.0);
        self
    }

    pub fn top_center(mut self) -> Self {
        self.x = nf32!(0.5);
        self.y = nf32!(0.0);
        self
    }

    pub fn top_right(mut self) -> Self {
        self.x = nf32!(1.0);
        self.y = nf32!(0.0);
        self
    }

    pub fn middle_left(mut self) -> Self {
        self.x = nf32!(0.0);
        self.y = nf32!(0.5);
        self
    }

    pub fn center(mut self) -> Self {
        self.x = nf32!(0.5);
        self.y = nf32!(0.5);
        self
    }

    pub fn middle_right(mut self) -> Self {
        self.x = nf32!(1.0);
        self.y = nf32!(0.5);
        self
    }

    pub fn bottom_left(mut self) -> Self {
        self.x = nf32!(0.0);
        self.y = nf32!(1.0);
        self
    }

    pub fn bottom_center(mut self) -> Self {
        self.x = nf32!(0.5);
        self.y = nf32!(1.0);
        self
    }

    pub fn bottom_right(mut self) -> Self {
        self.x = nf32!(1.0);
        self.y = nf32!(1.0);
        self
    }

    //

    fn into_pattern(self) -> Option<Pattern> {
        Pattern::build(|out| {
            ElementWriter::new(out, "image")?
                .attr("href", (self.source,))?
                .attr("preserveAspectRatio", "none")?
                .attrs([
                    ("x", (1.0 - self.width.get()) * self.x.get()),
                    ("y", (1.0 - self.height.get()) * self.y.get()),
                    ("width", self.width.get()),
                    ("height", self.height.get()),
                ])?
                .attr("crossorigin", self.cross_origin.map(|x| (x,)))?
                .close()
        })
        .map(|pat| {
            pat.pattern_units(PatternUnits::ObjectBoundingBox)
                .pattern_content_units(PatternContentUnits::ObjectBoundingBox)
                .width(Length::units(1.0))
                .height(Length::units(1.0))
        })
        .ok()
    }
}

//

impl From<Color> for Paint {
    #[inline]
    fn from(value: Color) -> Self {
        Paint::color(value)
    }
}

impl From<LinearGradient> for Paint {
    #[inline]
    fn from(value: LinearGradient) -> Self {
        Paint::linear_gradient(value)
    }
}

impl From<RadialGradient> for Paint {
    #[inline]
    fn from(value: RadialGradient) -> Self {
        Paint::radial_gradient(value)
    }
}

impl From<Pattern> for Paint {
    #[inline]
    fn from(value: Pattern) -> Self {
        Paint::pattern(value)
    }
}

impl From<ImagePaint> for Paint {
    #[inline]
    fn from(value: ImagePaint) -> Self {
        Paint::image(value)
    }
}

//

#[derive(Debug, Clone, SmartDefault)]
pub struct PaintLayer {
    pub(crate) paint: Paint,
    pub(crate) blend_mode: BlendMode,
    #[default(NormalizedF32::ONE)]
    pub(crate) opacity: NormalizedF32,
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
    T: Into<Paint>,
{
    #[inline]
    fn from(value: T) -> Self {
        Self {
            paint: value.into(),
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

    fn needs_isolation(&self) -> bool {
        self.0.iter().any(|x| !x.blend_mode.is_default())
    }

    pub(crate) fn render<'a, W, D, S, L, G>(
        &self,
        ctx: &mut RenderContext<W>,
        draw_single_layer: D,
        draw_cached_layer: S,
        visit_layer: L,
        visit_group: G,
    ) -> std::fmt::Result
    where
        W: Write,
        D: FnOnce(&mut W) -> std::fmt::Result,
        S: FnOnce(&mut String) -> std::fmt::Result,
        L: Fn(
            ElementWriter<W, Initialized>,
            bool,
        ) -> Result<ElementWriter<W, Initialized>, std::fmt::Error>,
        G: Fn(
            ElementWriter<W, Initialized>,
        ) -> Result<ElementWriter<W, Initialized>, std::fmt::Error>,
    {
        if self.is_none() {
            return Ok(());
        }

        let layers = &self.0;
        let render_layer =
            |element: ElementWriter<W, Initialized>, layer: &PaintLayer, cached: bool| {
                visit_layer(
                    element
                        .attr("fill", (&layer.paint,))?
                        .attr_if(
                            "fill-opacity",
                            layer.opacity,
                            layer.opacity != NormalizedF32::ONE,
                        )?
                        .attr_if(
                            "style",
                            (format_args!("mix-blend-mode:{}", layer.blend_mode),),
                            !layer.blend_mode.is_default(),
                        )?,
                    cached,
                )?
                .close()
            };

        if layers.len() == 1 {
            render_layer(
                ElementWriter::new(ctx.out, "path")?.write_attr("d", draw_single_layer)?,
                &layers[0],
                false,
            )?;

            return Ok(());
        }

        let path = Path::build(draw_cached_layer)?;
        let href = format_args!("#{}", path.iri());
        ctx.resources.lock().get_or_add_resource(path.into());

        visit_group(ElementWriter::new(ctx.out, "g")?.attr_if(
            "style",
            "isolation:isolate",
            self.needs_isolation(),
        )?)?
        .content(|out| {
            layers.iter().try_for_each(|layer| {
                render_layer(
                    ElementWriter::new(out, "use")?.attr("href", (href,))?,
                    layer,
                    true,
                )
            })
        })?
        .close()?;

        Ok(())
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
