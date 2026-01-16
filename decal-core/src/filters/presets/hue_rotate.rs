use crate::filters::Filter;

// https://www.w3.org/TR/filter-effects-1/#huerotateEquivalent

pub fn hue_rotate(angle: f32) -> Filter {
    Filter::new(|ctx| {
        ctx.color_matrix().hue_rotate(angle).finish();
    })
}
