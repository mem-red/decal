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
    let (mut h_tl, mut v_tl) = (
        r.top_left.resolve_abs(w).unwrap_or_default(),
        r.top_left.resolve_abs(h).unwrap_or_default(),
    );
    let (mut h_tr, mut v_tr) = (
        r.top_right.resolve_abs(w).unwrap_or_default(),
        r.top_right.resolve_abs(h).unwrap_or_default(),
    );
    let (mut h_br, mut v_br) = (
        r.bottom_right.resolve_abs(w).unwrap_or_default(),
        r.bottom_right.resolve_abs(h).unwrap_or_default(),
    );
    let (mut h_bl, mut v_bl) = (
        r.bottom_left.resolve_abs(w).unwrap_or_default(),
        r.bottom_left.resolve_abs(h).unwrap_or_default(),
    );

    // scale all the radii by min ratio if adjacent radii exceed the available edge
    // length
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attributes::{
        CornerRadius,
        IntoCornerRadius,
    };

    fn corner_radius<I>(value: I) -> CornerRadius
    where
        I: IntoCornerRadius,
    {
        value.into_corner_radius().unwrap()
    }

    #[test]
    fn defaults_to_zero() {
        let r = compute_scaled_radii(CornerRadius::default(), 100.0, 50.0);
        assert_eq!(r.h_tl, 0.0);
        assert_eq!(r.v_tl, 0.0);
        assert_eq!(r.h_tr, 0.0);
        assert_eq!(r.v_tr, 0.0);
        assert_eq!(r.h_br, 0.0);
        assert_eq!(r.v_br, 0.0);
        assert_eq!(r.h_bl, 0.0);
        assert_eq!(r.v_bl, 0.0);
    }

    #[test]
    fn radii_within_bounds_are_not_scaled() {
        // top-left, top-right, bottom-right, bottom-left
        let r = compute_scaled_radii(corner_radius([10, 11, 12, 13]), 100.0, 100.0);
        assert_eq!(r.h_tl, 10.0);
        assert_eq!(r.v_tl, 10.0);
        assert_eq!(r.h_tr, 11.0);
        assert_eq!(r.v_tr, 11.0);
        assert_eq!(r.h_br, 12.0);
        assert_eq!(r.v_br, 12.0);
        assert_eq!(r.h_bl, 13.0);
        assert_eq!(r.v_bl, 13.0);
    }

    #[test]
    fn horizontal_radii_scales_when_exceeding_width() {
        // top-left + top-right > width
        let r = compute_scaled_radii(corner_radius([40, 40, 0, 0]), 40.0, 100.0);
        assert_eq!(r.h_tl, 20.0);
        assert_eq!(r.v_tl, 20.0);
        assert_eq!(r.h_tr, 20.0);
        assert_eq!(r.v_tr, 20.0);
        assert_eq!(r.h_br, 0.0);
        assert_eq!(r.v_br, 0.0);
        assert_eq!(r.h_bl, 0.0);
        assert_eq!(r.v_bl, 0.0);
    }

    #[test]
    fn vertical_radii_scales_when_exceeding_height() {
        // top-left + bottom-left > height
        let r = compute_scaled_radii(corner_radius([40, 0, 0, 40]), 100.0, 40.0);
        assert_eq!(r.h_tl, 20.0);
        assert_eq!(r.v_tl, 20.0);
        assert_eq!(r.h_tr, 0.0);
        assert_eq!(r.v_tr, 0.0);
        assert_eq!(r.h_br, 0.0);
        assert_eq!(r.v_br, 0.0);
        assert_eq!(r.h_bl, 20.0);
        assert_eq!(r.v_bl, 20.0);
    }

    #[test]
    fn minimum_scale_factor() {
        // min scale from width, height
        let r = compute_scaled_radii(corner_radius([40, 40, 40, 40]), 80.0, 20.0);
        assert_eq!(r.h_tl, 10.0);
        assert_eq!(r.v_tl, 10.0);
        assert_eq!(r.h_tr, 10.0);
        assert_eq!(r.v_tr, 10.0);
        assert_eq!(r.h_br, 10.0);
        assert_eq!(r.v_br, 10.0);
        assert_eq!(r.h_bl, 10.0);
        assert_eq!(r.v_bl, 10.0);
    }
}
