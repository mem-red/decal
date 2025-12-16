use crate::prelude::*;

#[derive(Debug, Clone)]
pub(crate) struct Appearance {
    pub background: Fill,
    pub border_color: Fill,
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
