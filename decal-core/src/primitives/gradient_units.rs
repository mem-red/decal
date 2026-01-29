use crate::utils::IsDefault;
use enum_display::EnumDisplay;

/// The coordinate system used to interpret gradient positions and lengths.
///
/// # Reference
///
/// https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/gradientUnits
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum GradientUnits {
    /// Coordinates are relative to the bounding box of the painted object.
    #[default]
    #[display("objectBoundingBox")]
    ObjectBoundingBox,
    /// Coordinates are interpreted in the current user coordinate system.
    #[display("userSpaceOnUse")]
    UserSpaceOnUse,
}

impl IsDefault for GradientUnits {}
