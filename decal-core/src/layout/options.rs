use tiny_skia::Transform;
use usvg::{ImageRendering, ShapeRendering, TextRendering};

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

#[derive(Debug, Clone, Default)]
pub struct RasterizeOptions {
    pub debug: bool,
    pub vectorize_options: VectorizeOptions,
    pub shape_rendering: ShapeRendering,
    pub text_rendering: TextRendering,
    pub image_rendering: ImageRendering,
    pub root_transform: Transform,
}
