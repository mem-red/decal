use crate::layout::font::FontRegistry;
use crate::layout::{Decal, RasterizeError, RasterizeOptions, VectorizeError, VectorizeOptions};
use lru::LruCache;
use parking_lot::Mutex;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tiny_skia::Pixmap;

pub(crate) type ImageCache = Arc<Mutex<LruCache<String, Arc<Vec<u8>>>>>;

const DEFAULT_IMAGE_CACHE_CAP: NonZeroUsize = NonZeroUsize::new(128).expect("128 is non-zero");

#[derive(Debug)]
pub struct EngineOptions {
    pub fonts: FontRegistry,
    pub image_cache_capacity: NonZeroUsize,
}

impl Default for EngineOptions {
    fn default() -> Self {
        Self {
            fonts: FontRegistry::default(),
            image_cache_capacity: DEFAULT_IMAGE_CACHE_CAP,
        }
    }
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
    ) -> Result<Pixmap, RasterizeError> {
        decal.set_fonts(self.fonts.clone());
        decal.compute_layout();
        decal.rasterize(&self.image_cache, options)
    }

    pub fn vectorize(
        &mut self,
        decal: &mut Decal,
        options: &VectorizeOptions,
    ) -> Result<String, VectorizeError> {
        decal.set_fonts(self.fonts.clone());
        decal.compute_layout();
        decal.vectorize(options)
    }
}
