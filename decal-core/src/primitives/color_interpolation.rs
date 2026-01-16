use enum_display::EnumDisplay;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, EnumDisplay)]
pub enum ColorInterpolation {
    #[display("sRGB")]
    SRgb,
    #[display("linearRGB")]
    LinearRgb,
}
