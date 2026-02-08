use crate::filters::Filter;

impl Filter {
    /// Creates a saturation filter effect with the specified intensity.
    ///
    /// # Arguments
    /// - `amount`: The saturation multiplier where `0.0` produces a fully
    ///   desaturated image and `1.0` leaves saturation unchanged.
    ///
    /// # Returns
    /// - [`Filter`] applying a saturation adjustment effect.
    ///
    /// # Reference
    ///
    /// https://www.w3.org/TR/filter-effects-1/#saturateEquivalent
    pub fn saturate(amount: f32) -> Self {
        Self::new(|ctx| {
            ctx.color_matrix().saturate(amount).finish();
        })
    }
}
