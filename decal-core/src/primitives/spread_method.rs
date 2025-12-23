use crate::utils::IsDefault;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum SpreadMethod {
    #[default]
    Pad,
    Reflect,
    Repeat,
}

impl IsDefault for SpreadMethod {}

impl Display for SpreadMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SpreadMethod::Pad => "pad",
            SpreadMethod::Reflect => "reflect",
            SpreadMethod::Repeat => "repeat",
        })
    }
}
