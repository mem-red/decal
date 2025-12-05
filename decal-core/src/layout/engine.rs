use crate::layout::font::FontRegistry;
use crate::layout::{Decal, RasterizationError, VectorizationError};
use std::sync::{Arc, Mutex};
use tiny_skia::{Pixmap, Transform};
use usvg::Options;

#[derive(Debug)]
pub struct EngineOptions<F>
where
    F: Into<FontRegistry>,
{
    pub fonts: F,
}

#[derive(Debug)]
pub struct Engine {
    fonts: Arc<Mutex<FontRegistry>>,
}

impl Engine {
    pub fn new<F>(options: EngineOptions<F>) -> Self
    where
        F: Into<FontRegistry>,
    {
        Self {
            fonts: Arc::new(Mutex::new(options.fonts.into())),
        }
    }

    pub fn append_font<T>(&mut self, alias: &str, data: T)
    where
        T: Into<Vec<u8>>,
    {
        if let Ok(mut registry) = self.fonts.lock() {
            registry.append_font(alias, data);
        }
    }

    pub fn rasterize(
        &mut self,
        decal: &mut Decal,
        options: Option<Options>,
        transform: Option<Transform>,
    ) -> Result<Pixmap, RasterizationError> {
        decal.set_fonts(self.fonts.clone());
        decal.compute_layout();
        decal.rasterize(options, transform)
    }

    pub fn vectorize(&mut self, decal: &mut Decal) -> Result<String, VectorizationError> {
        decal.set_fonts(self.fonts.clone());
        decal.compute_layout();
        decal.vectorize()
    }
}
