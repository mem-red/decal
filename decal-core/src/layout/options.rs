use tiny_skia::Transform;
use usvg::{ImageRendering, ShapeRendering, TextRendering};

#[derive(Debug, Copy, Clone, Default)]
pub struct VectorizeOptions {
    /// Write `width` and `height` attributes to SVG
    pub emit_svg_dimensions: bool,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RasterizeOptions {
    pub debug: bool,
    pub vectorize_options: VectorizeOptions,
    pub shape_rendering: ShapeRendering,
    pub text_rendering: TextRendering,
    pub image_rendering: ImageRendering,
    pub root_transform: Transform,
}
