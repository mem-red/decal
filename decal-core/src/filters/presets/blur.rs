use crate::filters::Filter;

pub fn blur(amount: f32) -> Filter {
    Filter::new(|ctx| {
        ctx.gaussian_blur().std_deviation(amount).finish();
    })
}
