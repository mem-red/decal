use crate::filters::Filter;
use crate::filters::primitives::TransferFunction;

// https://www.w3.org/TR/filter-effects-1/#contrastEquivalent

pub fn contrast(amount: f32) -> Filter {
    let intercept = -(0.5 * amount) + 0.5;

    Filter::new(|ctx| {
        ctx.component_transfer()
            .func_r(TransferFunction::linear(amount, intercept))
            .func_g(TransferFunction::linear(amount, intercept))
            .func_b(TransferFunction::linear(amount, intercept))
            .finish();
    })
}
