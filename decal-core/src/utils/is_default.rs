pub(crate) trait IsDefault: Default + PartialEq {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}
