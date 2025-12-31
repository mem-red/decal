use crate::utils::IsDefault;
use enum_display::EnumDisplay;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum EdgeMode {
    #[default]
    #[display("duplicate")]
    Duplicate,
    #[display("wrap")]
    Wrap,
    #[display("none")]
    None,
}

impl IsDefault for EdgeMode {}
