use crate::{
    layout::{
        Decal,
        RasterizeError,
        RasterizeOptions,
        Scene,
        VectorizeError,
        VectorizeOptions,
        font::FontRegistry,
    },
    primitives::Size,
};
use lru::LruCache;
use parking_lot::Mutex;
use smart_default::SmartDefault;
use std::{
    fmt::Write,
    num::NonZeroUsize,
    sync::Arc,
};
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
        scene: &mut Scene,
        options: &RasterizeOptions,
    ) -> Result<(Pixmap, Size<f32>), RasterizeError> {
        self.prepare(scene).rasterize(&self.image_cache, options)
    }

    pub fn vectorize(
        &mut self,
        scene: &mut Scene,
        options: &VectorizeOptions,
    ) -> Result<(String, Size<f32>), VectorizeError> {
        self.prepare(scene).vectorize(options)
    }

    pub fn stream_vector<T>(
        &mut self,
        destination: &mut T,
        scene: &mut Scene,
        options: &VectorizeOptions,
    ) -> Result<Size<f32>, VectorizeError>
    where
        T: Write,
    {
        self.prepare(scene).stream_vector(destination, options)
    }

    /// Prepares the scene for rendering.
    ///
    /// This injects the engine font registry and computes layout.
    fn prepare<'a>(&self, scene: &'a mut Scene) -> &'a mut Scene {
        scene.set_fonts(self.fonts.clone());
        scene.compute_layout();
        scene
    }
}
