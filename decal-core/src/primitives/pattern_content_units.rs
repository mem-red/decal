use crate::utils::IsDefault;
use enum_display::EnumDisplay;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default, EnumDisplay)]
pub enum PatternContentUnits {
    #[display("objectBoundingBox")]
    ObjectBoundingBox,
    #[default]
    #[display("userSpaceOnUse")]
    UserSpaceOnUse,
}

impl IsDefault for PatternContentUnits {}
