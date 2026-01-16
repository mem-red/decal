use crate::filters::Filter;
use crate::filters::primitives::TransferFunction;

// https://www.w3.org/TR/filter-effects-1/#invertEquivalent

pub fn invert(amount: f32) -> Filter {
    let a = amount.clamp(0.0, 1.0);
    let ia = 1.0 - a;

    Filter::new(|ctx| {
        ctx.component_transfer()
            .func_r(TransferFunction::table(vec![a, ia]))
            .func_g(TransferFunction::table(vec![a, ia]))
            .func_b(TransferFunction::table(vec![a, ia]))
            .finish();
    })
}
