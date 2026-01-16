use crate::filters::Filter;
use crate::primitives::Color;

// https://www.w3.org/TR/filter-effects-1/#dropshadowEquivalent

pub fn drop_shadow(dx: f32, dy: f32, blur_amount: f32, color: Option<Color>) -> Filter {
    Filter::new(|ctx| {
        ctx.drop_shadow()
            .dx(dx)
            .dy(dy)
            .std_deviation(blur_amount)
            .flood_color(color)
            .finish();
    })
}
