use crate::utils::IsDefault;
use enum_display::EnumDisplay;

/// Controls how pixels outside the bounds of an input image are handled.
///
/// # Reference
///
/// https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/edgeMode
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum EdgeMode {
    /// Extends the input image along each of its borders as necessary by
    /// duplicating the color values at the given edge of the input image.
    #[default]
    #[display("duplicate")]
    Duplicate,
    /// Extends the input image by taking the color values from the opposite
    /// edge of the image.
    #[display("wrap")]
    Wrap,
    /// Treats pixels outside the image as transparent.
    #[display("none")]
    None,
}

impl IsDefault for EdgeMode {}
