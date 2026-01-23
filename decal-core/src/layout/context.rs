use crate::{
    layout::{
        FontRegistry,
        VectorizeOptions,
    },
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
    pub(crate) root_size: Size<f32>,
    pub(crate) options: &'a VectorizeOptions,
}

impl<'a, T> RenderContext<'a, T>
where
    T: Write,
{
    #[cfg(test)]
    pub(crate) fn new(
        out: &'a mut T,
        resources: &'a Mutex<Resources>,
        options: &'a VectorizeOptions,
    ) -> Self {
        Self {
            out,
            fonts: Default::default(),
            resources,
            root_size: Size::from_values(0.0, 0.0),
            options,
        }
    }
}
