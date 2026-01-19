use crate::filters::{
    Filter,
    primitives::TransferFunction,
};

// https://www.w3.org/TR/filter-effects-1/#brightnessEquivalent

pub fn brightness(amount: f32) -> Filter {
    Filter::new(|ctx| {
        ctx.component_transfer()
            .func_r(TransferFunction::linear(amount, 0.0))
            .func_g(TransferFunction::linear(amount, 0.0))
            .func_b(TransferFunction::linear(amount, 0.0))
            .finish();
    })
}
