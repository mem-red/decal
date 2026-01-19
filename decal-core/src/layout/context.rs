use crate::layout::{FontRegistry, VectorizeOptions};
use crate::paint::Resources;
use crate::primitives::Size;
use parking_lot::Mutex;
use std::fmt::Write;
use std::sync::Arc;

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
