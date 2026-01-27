use crate::{
    filters::Filter,
    primitives::Color,
};

/// Creates a drop shadow filter effect with the specified offset, blur, and
/// color.
///
/// # Arguments
/// - `dx`: The horizontal offset of the shadow.
/// - `dy`: The vertical offset of the shadow.
/// - `blur_amount`: The gaussian standard deviation used to blur the shadow.
/// - `color`: The optional [`Color`] of the shadow.
///
/// # Returns
/// - [`Filter`] applying a drop shadow effect.
///
/// # Reference
///
/// https://www.w3.org/TR/filter-effects-1/#dropshadowEquivalent
pub fn drop_shadow<T>(dx: f32, dy: f32, blur_amount: f32, color: T) -> Filter
where
    T: Into<Option<Color>>,
{
    Filter::new(|ctx| {
        ctx.drop_shadow()
            .dx(dx)
            .dy(dy)
            .std_deviation(blur_amount)
            .flood_color(color)
            .finish();
    })
}
