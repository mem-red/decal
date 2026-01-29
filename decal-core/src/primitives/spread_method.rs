use crate::utils::IsDefault;
use enum_display::EnumDisplay;

/// Specifies how a gradient is extended beyond its normal bounds.
///
/// # Reference
///
/// https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/spreadMethod
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum SpreadMethod {
    /// The edge colors of the gradient are extended to fill the remaining area.
    #[default]
    #[display("pad")]
    Pad,
    /// The gradient is repeated in reverse order beyond its edges.
    #[display("reflect")]
    Reflect,
    /// The gradient is repeated in the same direction indefinitely.
    #[display("repeat")]
    Repeat,
}

impl IsDefault for SpreadMethod {}
