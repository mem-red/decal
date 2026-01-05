use std::f32::consts::PI;

const CX: f32 = 0.5;
const CY: f32 = 0.5;

// https://observablehq.com/@danburzo/css-gradient-line

pub(crate) fn angle_to_line(angle: f32) -> (f32, f32, f32, f32) {
    let rad = (90.0 - angle) * PI / 180.0;
    let cos = rad.cos();
    let sin = rad.sin();
    let scale = 0.5 / cos.abs().max(sin.abs()); // cover the box

    let dx = cos * scale;
    let dy = sin * scale;

    let x1 = CX - dx;
    let y1 = CY + dy; // y-axis is down
    let x2 = CX + dx;
    let y2 = CY - dy;

    (x1, y1, x2, y2)
}
