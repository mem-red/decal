use crate::utils::IsDefault;
use enum_display::EnumDisplay;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum MaskType {
    #[default]
    #[display("luminance")]
    Luminance,
    #[display("alpha")]
    Alpha,
}

impl IsDefault for MaskType {}
