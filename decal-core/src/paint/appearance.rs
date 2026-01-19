use crate::{
    attributes::CornerRadius,
    filters::Filter,
    primitives::{
        BlendMode,
        PaintStack,
        Transform,
    },
};
use smart_default::SmartDefault;

#[derive(Debug, Clone, SmartDefault)]
pub struct Appearance {
    pub(crate) background: PaintStack,
    pub(crate) blend_mode: BlendMode,
    pub(crate) border: PaintStack,
    pub(crate) corner_radius: CornerRadius,
    pub(crate) transform: Transform,
    #[default(true)]
    pub(crate) visible: bool,
    #[default(1.0)]
    pub(crate) opacity: f32,
    pub(crate) filter: Filter,
}
