use crate::capabilities::CornerRadius;
use crate::primitives::{Paint, Transform};

#[derive(Debug, Clone)]
pub(crate) struct Appearance {
    pub background: Paint,
    pub border_color: Paint,
    pub corner_radius: CornerRadius,
    pub transform: Transform,
    pub visible: bool,
    pub opacity: f32,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            background: Default::default(),
            border_color: Default::default(),
            corner_radius: Default::default(),
            transform: Default::default(),
            visible: true,
            opacity: 1.0,
        }
    }
}
