use crate::attributes::CornerRadius;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ScaledRadii {
    pub(crate) h_tl: f32,
    pub(crate) v_tl: f32,
    pub(crate) h_tr: f32,
    pub(crate) v_tr: f32,
    pub(crate) h_br: f32,
    pub(crate) v_br: f32,
    pub(crate) h_bl: f32,
    pub(crate) v_bl: f32,
}

pub(crate) fn compute_scaled_radii(r: CornerRadius, w: f32, h: f32) -> ScaledRadii {
    let (mut h_tl, mut v_tl) = (r.top_left.resolve_abs(w), r.top_left.resolve_abs(h));
    let (mut h_tr, mut v_tr) = (r.top_right.resolve_abs(w), r.top_right.resolve_abs(h));
    let (mut h_br, mut v_br) = (r.bottom_right.resolve_abs(w), r.bottom_right.resolve_abs(h));
    let (mut h_bl, mut v_bl) = (r.bottom_left.resolve_abs(w), r.bottom_left.resolve_abs(h));

    // scale all the radii by min ratio if adjacent radii exceed the available edge length
    let mut scale = 1.0_f32;
    let sum_top = h_tl + h_tr;
    let sum_bottom = h_bl + h_br;
    let sum_left = v_tl + v_bl;
    let sum_right = v_tr + v_br;

    if sum_top > 0.0 {
        scale = scale.min(w / sum_top);
    }

    if sum_bottom > 0.0 {
        scale = scale.min(w / sum_bottom);
    }

    if sum_left > 0.0 {
        scale = scale.min(h / sum_left);
    }

    if sum_right > 0.0 {
        scale = scale.min(h / sum_right);
    }

    if scale != 1.0 {
        h_tl *= scale;
        h_tr *= scale;
        h_br *= scale;
        h_bl *= scale;
        v_tl *= scale;
        v_tr *= scale;
        v_br *= scale;
        v_bl *= scale;
    }

    ScaledRadii {
        h_tl,
        v_tl,
        h_tr,
        v_tr,
        h_br,
        v_br,
        h_bl,
        v_bl,
    }
}
