use enum_display::EnumDisplay;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, EnumDisplay)]
pub enum CrossOrigin {
    #[display("anonymous")]
    Anonymous,
    #[display("use-credentials")]
    UseCredentials,
}
