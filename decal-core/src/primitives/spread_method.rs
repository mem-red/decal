use crate::utils::IsDefault;
use enum_display::EnumDisplay;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default, EnumDisplay)]
pub enum SpreadMethod {
    #[default]
    #[display("pad")]
    Pad,
    #[display("reflect")]
    Reflect,
    #[display("repeat")]
    Repeat,
}

impl IsDefault for SpreadMethod {}
