use crate::utils::IsDefault;
use enum_display::EnumDisplay;

/// The coordinate system used to interpret filter region geometry.
///
/// # Reference
///
/// https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/filterUnits
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum FilterUnits {
    /// Coordinates are relative to the bounding box of the filtered element.
    #[default]
    #[display("objectBoundingBox")]
    ObjectBoundingBox,
    /// Coordinates are interpreted in the current user coordinate system.
    #[display("userSpaceOnUse")]
    UserSpaceOnUse,
}

impl IsDefault for FilterUnits {}
