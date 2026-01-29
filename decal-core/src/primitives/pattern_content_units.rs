use crate::utils::IsDefault;
use enum_display::EnumDisplay;

/// The coordinate system used for the contents of a pattern.
///
/// # Reference
///
/// https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/patternContentUnits
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum PatternContentUnits {
    /// Coordinates are relative to the bounding box of the painted object.
    #[display("objectBoundingBox")]
    ObjectBoundingBox,
    /// Coordinates are interpreted in the current user coordinate system.
    #[default]
    #[display("userSpaceOnUse")]
    UserSpaceOnUse,
}

impl IsDefault for PatternContentUnits {}
