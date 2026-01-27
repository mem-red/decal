use crate::filters::Filter;

/// Creates a sepia filter effect with the specified intensity.
///
/// # Arguments
/// - `amount`: The sepia intensity where `0.0` leaves the image unchanged and
///   `1.0` applies a full sepia tone.
///
/// # Returns
/// - [`Filter`] applying a sepia tone effect.
///
/// # Reference
///
/// https://www.w3.org/TR/filter-effects-1/#sepiaEquivalent
pub fn sepia(amount: f32) -> Filter {
    let x = 1.0 - amount.clamp(0.0, 1.0);

    Filter::new(|ctx| {
        ctx.color_matrix()
            .matrix([
                [
                    0.393 + 0.607 * x,
                    0.769 - 0.769 * x,
                    0.189 - 0.189 * x,
                    0.0,
                    0.0,
                ],
                [
                    0.349 - 0.349 * x,
                    0.686 + 0.314 * x,
                    0.168 - 0.168 * x,
                    0.0,
                    0.0,
                ],
                [
                    0.272 - 0.272 * x,
                    0.534 - 0.534 * x,
                    0.131 + 0.869 * x,
                    0.0,
                    0.0,
                ],
                [0.0, 0.0, 0.0, 1.0, 0.0],
            ])
            .finish();
    })
}
