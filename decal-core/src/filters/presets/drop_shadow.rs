use crate::filters::Filter;
use crate::filters::primitives::TransferFunction;

// https://www.w3.org/TR/filter-effects-1/#dropshadowEquivalent

pub fn drop_shadow(dx: f32, dy: f32, blur_amount: f32) -> Filter {
    Filter::new(|ctx| {
        ctx.drop_shadow()
            .dx(dx)
            .dy(dy)
            .std_deviation(blur_amount)
            .finish();
    })
}
