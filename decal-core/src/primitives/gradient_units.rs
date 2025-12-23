use crate::utils::IsDefault;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum GradientUnits {
    UserSpaceOnUse,
    #[default]
    ObjectBoundingBox,
}

impl IsDefault for GradientUnits {}

impl Display for GradientUnits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GradientUnits::UserSpaceOnUse => "userSpaceOnUse",
            GradientUnits::ObjectBoundingBox => "objectBoundingBox",
        })
    }
}
