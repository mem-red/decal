use crate::{
    layout::FontRegistry,
    paint::Resources,
    primitives::Size,
};
use parking_lot::Mutex;
use std::{
    fmt::Write,
    sync::Arc,
};

#[derive(Debug)]
pub(crate) struct RenderContext<'a, T>
where
    T: Write,
{
    pub(crate) out: &'a mut T,
    pub(crate) fonts: Arc<Mutex<FontRegistry>>,
    pub(crate) resources: &'a Mutex<Resources>,
    pub(crate) scene_size: Size<f32>,
}

impl<'a, T> RenderContext<'a, T>
where
    T: Write,
{
    #[cfg(test)]
    pub(crate) fn new(out: &'a mut T, resources: &'a Mutex<Resources>) -> Self {
        Self {
            out,
            fonts: Default::default(),
            resources,
            scene_size: Size::from_values(0.0, 0.0),
        }
    }
}
