use enum_display::EnumDisplay;

#[derive(Debug, Copy, Clone, EnumDisplay)]
pub enum CrossOrigin {
    #[display("anonymous")]
    Anonymous,
    #[display("use-credentials")]
    UseCredentials,
}
