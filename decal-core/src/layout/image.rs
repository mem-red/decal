use crate::primitives::CrossOrigin;
use enum_display::EnumDisplay;
use quick_xml::escape::escape;
use taffy::Size;

#[derive(Debug, Clone, EnumDisplay)]
pub enum ImageSource {
    #[display("{0}")]
    Url(String),
    #[display("{0}")]
    DataUri(String),
    // TODO support raw image data
    // #[display("{0}")]
    // Binary(u64, Vec<u8>), // (img_key, data)
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

    // TODO
    // pub fn binary<T>(data: T) -> Self
    // where
    //     T: Into<Vec<u8>>,
    // {
    //     ImageSource::Binary(0, data.into())
    // }

    pub fn svg<S>(svg: S) -> Self
    where
        S: Into<String>,
    {
        ImageSource::Svg(svg.into())
    }
}

impl Default for ImageSource {
    fn default() -> Self {
        ImageSource::url("")
    }
}

impl From<String> for ImageSource {
    fn from(value: String) -> Self {
        ImageSource::url(value)
    }
}

impl From<&str> for ImageSource {
    fn from(value: &str) -> Self {
        ImageSource::url(value)
    }
}
