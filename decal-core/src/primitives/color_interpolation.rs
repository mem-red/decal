use enum_display::EnumDisplay;

/// The color space used for interpolating colors in gradients and filters.
///
/// Defaults to:
/// - [`Srgb`](Self::SRgb) for gradients.
/// - [`LinearRgb`](Self::LinearRgb) for filters.
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, EnumDisplay)]
pub enum ColorInterpolation {
    /// Performs color interpolation in the standard sRGB color space.
    ///
    /// Default value for gradients.
    #[display("sRGB")]
    SRgb,
    /// Performs color interpolation in linear RGB color space.
    ///
    /// Default value for filters.
    #[display("linearRGB")]
    LinearRgb,
}
