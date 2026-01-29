use crate::utils::IsDefault;
use enum_display::EnumDisplay;

/// Specifies how mask content is interpreted when compositing.
///
/// # Reference
///
/// https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/mask-type
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum MaskType {
    /// Uses the luminance of the mask content to determine masking.
    #[default]
    #[display("luminance")]
    Luminance,
    /// Uses the alpha channel of the mask content to determine masking.
    #[display("alpha")]
    Alpha,
}

impl IsDefault for MaskType {}
