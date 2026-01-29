use crate::utils::IsDefault;
use enum_display::EnumDisplay;

/// The coordinate system used by filter primitives.
///
/// # Reference
///
/// https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/primitiveUnits
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum PrimitiveUnits {
    /// Coordinates are relative to the bounding box of the filtered element.
    #[display("objectBoundingBox")]
    ObjectBoundingBox,
    /// Coordinates are interpreted in the current user coordinate system.
    #[default]
    #[display("userSpaceOnUse")]
    UserSpaceOnUse,
}

impl IsDefault for PrimitiveUnits {}
