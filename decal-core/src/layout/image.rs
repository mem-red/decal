use quick_xml::escape::escape;
use std::fmt::Display;
use taffy::Size;

#[derive(Debug, Clone)]
pub enum ImageSource {
    Url(String),
    DataUri(String),
    Svg(String),
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ImageMeta {
    pub(crate) source: ImageSource,
    pub(crate) width: f32,
    pub(crate) height: f32,
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
}

impl ImageSource {
    pub fn url<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        ImageSource::Url(escape(url.into()).to_string())
    }

    pub fn data_uri<S>(data_uri: S) -> Self
    where
        S: Into<String>,
    {
        ImageSource::DataUri(data_uri.into())
    }

    pub fn svg<S>(svg: S) -> Self
    where
        S: Into<String>,
    {
        ImageSource::Svg(svg.into())
    }
}

impl Default for ImageSource {
    fn default() -> Self {
        ImageSource::Url("".to_string())
    }
}

impl From<String> for ImageSource {
    fn from(value: String) -> Self {
        ImageSource::Url(escape(value).to_string())
    }
}

impl From<&str> for ImageSource {
    fn from(value: &str) -> Self {
        ImageSource::Url(escape(value).to_string())
    }
}

impl Display for ImageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ImageSource::Url(url) => url,
            ImageSource::DataUri(uri) => uri,
            ImageSource::Svg(svg) => svg,
        };

        write!(f, "{}", value)
    }
}
