use crate::utils::IsDefault;
use enum_display::EnumDisplay;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum FilterUnits {
    #[default]
    #[display("objectBoundingBox")]
    ObjectBoundingBox,
    #[display("userSpaceOnUse")]
    UserSpaceOnUse,
}

impl IsDefault for FilterUnits {}
