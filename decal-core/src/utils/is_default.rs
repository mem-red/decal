/// Helper trait for checking whether a value equals its default state.
pub(crate) trait IsDefault: Default + PartialEq {
    /// Returns `true` if the value is equal to its default.
    ///
    /// # Returns
    /// - `true` when `self == Self::default()`.
    /// - `false` otherwise.
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default, PartialEq)]
    enum Test {
        A,
        #[default]
        B,
    }

    impl IsDefault for Test {}

    #[test]
    fn true_for_default_instance() {
        let x = Test::default();
        assert!(x.is_default());
    }

    #[test]
    fn false_for_non_default_instance() {
        let x = Test::A;
        assert!(!x.is_default());
    }
}
