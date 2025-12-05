use crate::prelude::*;

#[derive(Debug, Clone)]
pub(crate) struct Appearance {
    pub background: Fill,
    pub border_color: Fill,
    pub corner_radius: CornerRadius,
    pub visible: bool,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            background: Default::default(),
            border_color: Default::default(),
            corner_radius: Default::default(),
            visible: true,
        }
    }
}
