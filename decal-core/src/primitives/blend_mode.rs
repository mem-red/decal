use crate::utils::IsDefault;
use enum_display::EnumDisplay;

/// Specifies how a node's pixels are blended with the backdrop (background).
///
/// # Reference
///
/// https://drafts.csswg.org/compositing-2
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum BlendMode {
    /// The node is rendered without any blending.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendingnormal
    #[default]
    #[display("normal")]
    Normal,
    /// Selects the darker of the backdrop and source colors. Areas lighter than
    /// the backdrop remain unchanged.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendingdarken
    #[display("darken")]
    Darken,
    /// Multiplies the source and backdrop colors.
    ///
    /// The result is always darker than or equal to either input. White
    /// preserves the original color; black results in black.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendingmultiply
    #[display("multiply")]
    Multiply,
    /// Darkens the backdrop color to reflect the source color.
    ///
    /// Painting with white produces no change.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendingcolorburn
    #[display("color-burn")]
    ColorBurn,
    /// Selects the lighter of the backdrop and source colors.
    ///
    /// Areas darker than the backdrop remain unchanged.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendinglighten
    #[display("lighten")]
    Lighten,
    /// Multiplies the complements of the source and backdrop colors.
    ///
    /// The result is always lighter than or equal to either input. Black
    /// preserves the original color; white results in white.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendingscreen
    #[display("screen")]
    Screen,
    /// Brightens the backdrop color based on the source color.
    ///
    /// Painting with black produces no change.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendingcolordodge
    #[display("color-dodge")]
    ColorDodge,
    /// Multiplies or screens colors depending on the backdrop.
    ///
    /// Preserves highlights and shadows of the backdrop while mixing in the
    /// source color.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendingoverlay
    #[display("overlay")]
    Overlay,
    /// Darkens or lightens colors depending on the source color.
    ///
    /// Produces a softer effect than [`HardLight`], similar to a diffused
    /// spotlight.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendingsoftlight
    ///
    /// [`HardLight`]: Self::HardLight
    #[display("soft-light")]
    SoftLight,
    /// Multiplies or screens colors depending on the source color.
    ///
    /// Produces a high-contrast effect similar to a harsh spotlight.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendinghardlight
    #[display("hard-light")]
    HardLight,
    /// Subtracts the darker color from the lighter one.
    ///
    /// Painting with white inverts the backdrop; black produces no change.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendingdifference
    #[display("difference")]
    Difference,
    /// Similar to [`Difference`] but with lower contrast.
    ///
    /// Produces softer inversion effects.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendingexclusion
    ///
    /// [`Difference`]: Self::Difference
    #[display("exclusion")]
    Exclusion,
    /// Uses the hue of the source color and the saturation and luminosity of
    /// the backdrop color.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendinghue
    #[display("hue")]
    Hue,
    /// Uses the saturation of the source color and the hue and luminosity of
    /// the backdrop color.
    ///
    /// Painting over a fully desaturated backdrop produces no change.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendingsaturation
    #[display("saturation")]
    Saturation,
    /// Uses the hue and saturation of the source color and the luminosity of
    /// the backdrop color.
    ///
    /// Useful for coloring monochrome images while preserving shading.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendingcolor
    #[display("color")]
    Color,
    /// Uses the luminosity of the source color and the hue and saturation of
    /// the backdrop color.
    ///
    /// Inverse of the [`Color`] blend mode.
    ///
    /// # Reference
    ///
    /// https://drafts.csswg.org/compositing-2/#blendingluminosity
    ///
    /// [`Color`]: Self::Color
    #[display("luminosity")]
    Luminosity,
}

impl IsDefault for BlendMode {}
