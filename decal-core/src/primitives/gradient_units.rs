use crate::utils::IsDefault;
use enum_display::EnumDisplay;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default, EnumDisplay)]
pub enum GradientUnits {
    #[default]
    #[display("objectBoundingBox")]
    ObjectBoundingBox,
    #[display("userSpaceOnUse")]
    UserSpaceOnUse,
}

impl IsDefault for GradientUnits {}
