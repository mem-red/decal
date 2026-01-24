use crate::utils::IsDefault;
use enum_display::EnumDisplay;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum PrimitiveUnits {
    #[display("objectBoundingBox")]
    ObjectBoundingBox,
    #[default]
    #[display("userSpaceOnUse")]
    UserSpaceOnUse,
}

impl IsDefault for PrimitiveUnits {}
