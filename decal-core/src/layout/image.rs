use taffy::Size;

#[derive(Debug, Clone, Default)]
pub(crate) struct ImageMeta {
    pub(crate) source: String,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) is_loaded: bool,
}

impl ImageMeta {
    pub(crate) fn new<S>(source: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            source: source.into(),
            ..Default::default()
        }
    }

    pub(crate) fn load(&mut self) {
        if !self.is_loaded {
            self.is_loaded = true;
            // TODO fetch image
        }
    }

    pub(crate) fn measure(&mut self, known_dimensions: Size<Option<f32>>) -> Size<f32> {
        self.load();
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
