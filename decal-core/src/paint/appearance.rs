use crate::capabilities::CornerRadius;
use crate::filters::Filter;
use crate::primitives::{Paint, Transform};

#[derive(Debug, Clone)]
pub struct Appearance {
    pub(crate) background: Paint,
    pub(crate) background_opacity: f32,
    pub(crate) border: Paint,
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
            background_opacity: 1.0,
            border: Default::default(),
            corner_radius: Default::default(),
            transform: Default::default(),
            visible: true,
            opacity: 1.0,
            filter: Default::default(),
        }
    }
}
