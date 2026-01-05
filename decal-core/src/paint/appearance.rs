use crate::capabilities::CornerRadius;
use crate::filters::Filter;
use crate::primitives::{BlendMode, PaintStack, Transform};

#[derive(Debug, Clone)]
pub struct Appearance {
    pub(crate) background: PaintStack,
    pub(crate) blend_mode: BlendMode,
    pub(crate) border: PaintStack,
    pub(crate) corner_radius: CornerRadius,
    pub(crate) transform: Transform,
    pub(crate) visible: bool,
    pub(crate) opacity: f32,
    pub(crate) filter: Filter,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            background: Default::default(),
            blend_mode: Default::default(),
            border: Default::default(),
            corner_radius: Default::default(),
            transform: Default::default(),
            visible: true,
            opacity: 1.0,
            filter: Default::default(),
        }
    }
}
