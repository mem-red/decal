use crate::utils::IsDefault;
use enum_display::EnumDisplay;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum BlendMode {
    #[default]
    #[display("normal")]
    Normal,
    #[display("darken")]
    Darken,
    #[display("multiply")]
    Multiply,
    #[display("color-burn")]
    ColorBurn,
    #[display("lighten")]
    Lighten,
    #[display("screen")]
    Screen,
    #[display("color-dodge")]
    ColorDodge,
    #[display("overlay")]
    Overlay,
    #[display("soft-light")]
    SoftLight,
    #[display("hard-light")]
    HardLight,
    #[display("difference")]
    Difference,
    #[display("exclusion")]
    Exclusion,
    #[display("hue")]
    Hue,
    #[display("saturation")]
    Saturation,
    #[display("color")]
    Color,
    #[display("luminosity")]
    Luminosity,
}

impl IsDefault for BlendMode {}
