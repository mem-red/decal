use crate::utils::IsDefault;
use enum_display::EnumDisplay;

/// The coordinate system used to position and size a pattern tile.
///
/// Reference
///
/// https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/patternUnits
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum PatternUnits {
    /// Coordinates are relative to the bounding box of the painted object.
    #[default]
    #[display("objectBoundingBox")]
    ObjectBoundingBox,
    /// Coordinates are interpreted in the current user coordinate system.
    #[display("userSpaceOnUse")]
    UserSpaceOnUse,
}

impl IsDefault for PatternUnits {}
