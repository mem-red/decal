use enum_display::EnumDisplay;

/// The CORS mode used when loading external images.
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, EnumDisplay)]
pub enum CrossOrigin {
    /// Requests the resource without credentials.
    #[display("anonymous")]
    Anonymous,
    /// Requests the resource with credentials included.
    #[display("use-credentials")]
    UseCredentials,
}
