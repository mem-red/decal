use std::{
    fmt::{
        Debug,
        Formatter,
    },
    sync::Arc,
};
use tiny_skia::Transform;
use usvg::{
    ImageKind,
    ImageRendering,
    ShapeRendering,
    TextRendering,
};

#[derive(Debug, Clone, Default)]
pub enum SvgDimensions {
    #[default]
    Omit,
    Layout,
    Custom {
        width: String,
        height: String,
    },
}

#[derive(Debug, Clone, Default)]
pub struct VectorizeOptions {
    pub svg_dimensions: SvgDimensions,
    pub omit_svg_xmlns: bool,
}

//

pub type ImageHrefDataResolver<'a> =
    Arc<dyn Fn(&str, Arc<Vec<u8>>, &usvg::Options) -> Option<ImageKind> + Send + Sync + 'a>;

pub type ImageHrefStringResolver<'a> =
    Arc<dyn Fn(&str, &usvg::Options) -> Option<ImageKind> + Send + Sync + 'a>;

#[derive(Clone, Default)]
pub struct ImageOptions<'a> {
    pub disable_caching: bool,
    pub cache_ignore_list: Vec<String>,
    pub cache_ignore_fn: Option<Arc<dyn Fn(&str) -> bool + Send + Sync>>,
    pub href_data_resolver: Option<ImageHrefDataResolver<'a>>,
    pub href_string_resolver: Option<ImageHrefStringResolver<'a>>,
}

impl<'a> Debug for ImageOptions<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageOptions")
            .field("disable_caching", &self.disable_caching)
            .field("cache_ignore_list", &self.cache_ignore_list)
            .finish()
    }
}

#[derive(Debug, Clone, Default)]
pub struct RasterizeOptions<'a> {
    pub debug: bool,
    pub vectorize_options: VectorizeOptions,
    pub shape_rendering: ShapeRendering,
    pub text_rendering: TextRendering,
    pub image_rendering: ImageRendering,
    pub root_transform: Transform,
    pub image: ImageOptions<'a>,
}
