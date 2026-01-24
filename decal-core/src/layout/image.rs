use crate::{
    layout::RenderContext,
    primitives::CrossOrigin,
    utils::ElementWriter,
};
use enum_display::EnumDisplay;
use quick_xml::escape::escape;
use std::fmt::Write;
use taffy::Size;

#[derive(Debug, Clone, EnumDisplay)]
pub enum ImageSource {
    #[display("{0}")]
    Href(String),
    #[display("{0}")]
    Svg(String),
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ImageMeta {
    pub(crate) source: ImageSource,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) cross_origin: Option<CrossOrigin>,
}

impl ImageMeta {
    pub(crate) fn new<S>(source: S, width: f32, height: f32) -> Self
    where
        S: Into<ImageSource>,
    {
        Self {
            source: source.into(),
            width,
            height,
            ..Default::default()
        }
    }

    pub(crate) fn measure(&mut self, known_dimensions: Size<Option<f32>>) -> Size<f32> {
        match (known_dimensions.width, known_dimensions.height) {
            (Some(width), Some(height)) => Size { width, height },
            (Some(width), None) => Size {
                width,
                height: (width / self.width.max(1.0)) * self.height,
            },
            (None, Some(height)) => Size {
                width: (height / self.height.max(1.0)) * self.width,
                height,
            },
            (None, None) => Size {
                width: self.width,
                height: self.height,
            },
        }
    }

    pub(crate) fn render<W>(
        &self,
        ctx: &mut RenderContext<W>,
        layout: taffy::Layout,
    ) -> std::fmt::Result
    where
        W: Write,
    {
        let Size { width, height } = layout.size;
        match &self.source {
            ImageSource::Href(href) => ElementWriter::new(ctx.out, "image")?
                .attr("href", href.as_str())?
                .attrs([("width", width), ("height", height)])?
                .attr("crossorigin", self.cross_origin.map(|x| (x,)))?
                .close(),
            ImageSource::Svg(svg) => ctx.out.write_str(svg.as_str()),
        }
    }
}

impl ImageSource {
    pub fn href<S>(url: S) -> Self
    where
        S: AsRef<str>,
    {
        ImageSource::Href(escape(url.as_ref()).to_string())
    }

    pub fn svg<S>(svg: S) -> Self
    where
        S: AsRef<str>,
    {
        ImageSource::Svg(svg.as_ref().to_string())
    }
}

impl Default for ImageSource {
    fn default() -> Self {
        ImageSource::href("")
    }
}

impl<T> From<T> for ImageSource
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        ImageSource::href(value.as_ref())
    }
}
