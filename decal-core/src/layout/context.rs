use crate::{
    layout::Scene,
    primitives::Size,
};
use std::fmt::Write;

#[derive(Debug)]
pub(crate) struct RenderContext<'a, T>
where
    T: Write,
{
    pub(crate) scene: &'a Scene,
    pub(crate) out: &'a mut T,
    pub(crate) scene_size: Size<f32>,
}

impl<'a, T> RenderContext<'a, T>
where
    T: Write,
{
    #[cfg(test)]
    pub(crate) fn new(out: &'a mut T, scene: &'a Scene) -> Self {
        Self {
            scene,
            out,
            scene_size: Size::from_values(0.0, 0.0),
        }
    }
}
