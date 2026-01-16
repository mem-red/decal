use crate::filters::Filter;

// https://www.w3.org/TR/filter-effects-1/#blurEquivalent

pub fn blur(amount: f32) -> Filter {
    Filter::new(|ctx| {
        ctx.gaussian_blur().std_deviation(amount).finish();
    })
}
