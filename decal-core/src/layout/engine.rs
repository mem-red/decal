use crate::layout::font::FontRegistry;
use crate::layout::{Decal, RasterizeError, RasterizeOptions, VectorizeError, VectorizeOptions};
use crate::primitives::Size;
use lru::LruCache;
use parking_lot::Mutex;
use smart_default::SmartDefault;
use std::fmt::Write;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tiny_skia::Pixmap;
use usvg::ImageKind;

pub(crate) type ImageCache = Arc<Mutex<LruCache<String, ImageKind>>>;

const DEFAULT_IMAGE_CACHE_CAP: NonZeroUsize = NonZeroUsize::new(128).expect("128 is non-zero");

#[derive(Debug, SmartDefault)]
pub struct EngineOptions {
    pub fonts: FontRegistry,
    #[default(DEFAULT_IMAGE_CACHE_CAP)]
    pub image_cache_capacity: NonZeroUsize,
}

#[derive(Debug)]
pub struct Engine {
    fonts: Arc<Mutex<FontRegistry>>,
    image_cache: ImageCache,
}

impl Engine {
    pub fn new(options: EngineOptions) -> Self {
        Self {
            fonts: Arc::new(Mutex::new(options.fonts)),
            image_cache: Arc::new(Mutex::new(LruCache::new(options.image_cache_capacity))),
        }
    }

    pub fn append_font<T>(&mut self, alias: &str, data: T)
    where
        T: Into<Vec<u8>>,
    {
        self.fonts.lock().append_font(alias, data);
    }

    pub fn rasterize(
        &mut self,
        decal: &mut Decal,
        options: &RasterizeOptions,
    ) -> Result<(Pixmap, Size<f32>), RasterizeError> {
        self.prepare(decal).rasterize(&self.image_cache, options)
    }

    pub fn vectorize(
        &mut self,
        decal: &mut Decal,
        options: &VectorizeOptions,
    ) -> Result<(String, Size<f32>), VectorizeError> {
        self.prepare(decal).vectorize(options)
    }

    pub fn stream_vector<T>(
        &mut self,
        destination: &mut T,
        decal: &mut Decal,
        options: &VectorizeOptions,
    ) -> Result<Size<f32>, VectorizeError>
    where
        T: Write,
    {
        self.prepare(decal).stream_vector(destination, options)
    }

    //

    fn prepare<'a>(&self, decal: &'a mut Decal) -> &'a mut Decal {
        decal.set_fonts(self.fonts.clone());
        decal.compute_layout();
        decal
    }
}
