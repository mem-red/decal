use crate::primitives::{
    MaskType,
    PaintStack,
};

#[derive(Debug, Copy, Clone, Default)]
pub enum StencilScope {
    VectorGlyphs,
    #[default]
    AllGlyphs,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum StencilType {
    #[default]
    Alpha,
    Luminance,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Stencil {
    pub(crate) paint: PaintStack,
    pub(crate) scope: StencilScope,
    pub(crate) r#type: StencilType,
}

impl Stencil {
    pub(crate) fn is_none(&self) -> bool {
        self.paint.is_none()
    }
}

impl From<StencilType> for MaskType {
    fn from(value: StencilType) -> Self {
        match value {
            StencilType::Alpha => MaskType::Alpha,
            StencilType::Luminance => MaskType::Luminance,
        }
    }
}
