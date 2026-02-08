use crate::filters::Filter;

impl Filter {
    /// Creates a hue rotation filter effect with the specified rotation angle.
    ///
    /// # Arguments
    /// - `angle`: The hue rotation angle in degrees.
    ///
    /// # Returns
    /// - [`Filter`] applying a hue rotation effect.
    ///
    /// # Reference
    ///
    /// https://www.w3.org/TR/filter-effects-1/#huerotateEquivalent
    pub fn hue_rotate(angle: f32) -> Self {
        Self::new(|ctx| {
            ctx.color_matrix().hue_rotate(angle).finish();
        })
    }
}
