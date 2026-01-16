use crate::filters::Filter;

// https://www.w3.org/TR/filter-effects-1/#saturateEquivalent

pub fn saturate(amount: f32) -> Filter {
    Filter::new(|ctx| {
        ctx.color_matrix().saturate(amount).finish();
    })
}
