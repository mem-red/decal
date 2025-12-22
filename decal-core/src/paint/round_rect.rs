use crate::paint::ScaledRadii;
use crate::utils::PathWriter;

pub(crate) fn write_fill_path<T>(out: &mut T, w: f32, h: f32, r: ScaledRadii) -> std::fmt::Result
where
    T: std::fmt::Write,
{
    write_round_rect(out, 0.0, 0.0, w, h, r)
}

pub(crate) fn write_border_path<T>(
    out: &mut T,
    w: f32,
    h: f32,
    r: ScaledRadii,
    borders: (f32, f32, f32, f32),
) -> std::fmt::Result
where
    T: std::fmt::Write,
{
    let (bt, br, bb, bl) = borders;
    if bt + br + bb + bl == 0.0 {
        return Ok(());
    }

    write_round_rect(out, 0.0, 0.0, w, h, r)?; // outer ring
    write_round_rect(out, bl, bt, w - br, h - bb, inset_radii(r, borders))?; // inner ring

    Ok(())
}

pub(crate) fn write_clip_path<T>(
    out: &mut T,
    w: f32,
    h: f32,
    r: ScaledRadii,
    borders: (f32, f32, f32, f32),
    clip_x: bool,
    clip_y: bool,
    root_size: (f32, f32),
) -> std::fmt::Result
where
    T: std::fmt::Write,
{
    let (bt, br, bb, bl) = borders;
    let (root_w, root_h) = root_size;
    let ir = inset_radii(r, borders);
    let (x1, y1) = (bl, bt);
    let (x2, y2) = (w - br, h - bb);

    match (clip_x, clip_y) {
        // no clipping
        (false, false) => Ok(()),
        // full clipping
        (true, true) => write_round_rect(out, x1, y1, x2, y2, ir),
        // clip x
        (true, false) => write_round_rect(out, x1, 0.0, x2, root_h, ir),
        // clip y
        (false, true) => write_round_rect(out, 0.0, y1, root_w, y2, ir),
    }
}

fn inset_radii(r: ScaledRadii, borders: (f32, f32, f32, f32)) -> ScaledRadii {
    let (bt, br, bb, bl) = borders;

    ScaledRadii {
        h_tl: (r.h_tl - bl).max(0.0),
        h_tr: (r.h_tr - br).max(0.0),
        h_br: (r.h_br - br).max(0.0),
        h_bl: (r.h_bl - bl).max(0.0),
        //
        v_tl: (r.v_tl - bt).max(0.0),
        v_tr: (r.v_tr - bt).max(0.0),
        v_br: (r.v_br - bb).max(0.0),
        v_bl: (r.v_bl - bb).max(0.0),
    }
}

fn write_round_rect<T>(
    out: &mut T,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    r: ScaledRadii,
) -> std::fmt::Result
where
    T: std::fmt::Write,
{
    if r.h_tl + r.h_tr + r.h_br + r.h_bl == 0.0 {
        PathWriter::new(out)
            .move_to(x1, y1)
            .horizontal_to(x2)
            .vertical_to(y2)
            .horizontal_to(x1)
            .close()
    } else {
        PathWriter::new(out)
            .move_to(x1 + r.h_tl, y1)
            .horizontal_to(x2 - r.h_tr)
            .arc_to(r.h_tr, r.v_tr, x2, y1 + r.v_tr)
            .vertical_to(y2 - r.v_br)
            .arc_to(r.h_br, r.v_br, x2 - r.h_br, y2)
            .horizontal_to(x1 + r.h_bl)
            .arc_to(r.h_bl, r.v_bl, x1, y2 - r.v_bl)
            .vertical_to(y1 + r.v_tl)
            .arc_to(r.h_tl, r.v_tl, x1 + r.h_tl, y1)
            .close()
    }
}
