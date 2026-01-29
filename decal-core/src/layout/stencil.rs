use crate::primitives::{
    MaskType,
    PaintStack,
};

/// Specifies which glyphs participate in stencil mask generation.
#[derive(Debug, Copy, Clone, Default)]
pub enum StencilScope {
    /// Includes only vector glyphs when generating the stencil mask.
    ///
    /// Bitmap glyphs such as emojis are rendered normally outside the mask.
    VectorGlyphs,
    /// Includes all glyphs when generating the stencil mask.
    #[default]
    AllGlyphs,
}

/// Specifies how the stencil mask is derived from glyphs.
#[derive(Debug, Copy, Clone, Default)]
pub enum StencilType {
    /// Uses the alpha channel of the glyphs to control stencil opacity.
    ///
    /// Transparent regions contribute less to the mask, while opaque regions
    /// contribute more.
    #[default]
    Alpha,
    /// Uses the luminance (brightness) of the glyphs to control stencil
    /// opacity.
    ///
    /// Brighter regions contribute more to the mask, regardless of
    /// transparency.
    Luminance,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Stencil {
    /// Stencil source/base.
    pub(crate) paint: PaintStack,
    pub(crate) scope: StencilScope,
    pub(crate) r#type: StencilType,
}

impl Stencil {
    /// Returns `true` if no stencil paint is configured.
    pub(crate) fn is_none(&self) -> bool {
        self.paint.is_none()
    }
}

impl From<StencilType> for MaskType {
    #[inline]
    fn from(value: StencilType) -> Self {
        match value {
            StencilType::Alpha => MaskType::Alpha,
            StencilType::Luminance => MaskType::Luminance,
        }
    }
}
