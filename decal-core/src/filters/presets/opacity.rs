use crate::filters::{
    Filter,
    primitives::TransferFunction,
};

// https://www.w3.org/TR/filter-effects-1/#opacityEquivalent

pub fn opacity(amount: f32) -> Filter {
    Filter::new(|ctx| {
        ctx.component_transfer()
            .func_a(TransferFunction::table(vec![0.0, amount]))
            .finish();
    })
}
