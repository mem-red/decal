use crate::filters::Filter;

pub fn hue_rotate(angle: f32) -> Filter {
    Filter::new(|ctx| {
        ctx.color_matrix().hue_rotate(angle).finish();
    })
}
