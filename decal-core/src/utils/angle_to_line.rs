use std::f32::consts::PI;

const CX: f32 = 0.5;
const CY: f32 = 0.5;

pub(crate) fn angle_to_line(angle: f32) -> (f32, f32, f32, f32) {
    let rad = (90.0 - angle) * PI / 180.0;
    let dx = rad.cos() / 2.0;
    let dy = rad.sin() / 2.0;
    let x1 = CX - dx;
    let y1 = CY + dy; // y-axis is down
    let x2 = CX + dx;
    let y2 = CY - dy;

    (x1, y1, x2, y2)
}
