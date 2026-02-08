use crate::filters::Filter;

impl Filter {
    /// Creates a gaussian blur filter effect with the specified standard
    /// deviation.
    ///
    /// # Arguments
    /// - `amount`: The gaussian standard deviation.
    ///
    /// # Returns
    /// - [`Filter`] applying a gaussian blur effect.
    ///
    /// # Reference
    ///
    /// https://www.w3.org/TR/filter-effects-1/#blurEquivalent
    pub fn blur(amount: f32) -> Self {
        Self::new(|ctx| {
            ctx.gaussian_blur().std_deviation(amount).finish();
        })
    }
}
