use std::f32::consts::PI;

const CX: f32 = 0.5;
const CY: f32 = 0.5;

/// Converts an angle in degrees into a normalized line segment (centered)
/// within a unit square.
///
/// # Arguments
/// - `angle`: The angle in degrees.
///
/// # Returns
/// - `(x1, y1, x2, y2)` representing the start and end points of the line in
///   normalized coordinates.
///
/// # Reference
///
/// https://observablehq.com/@danburzo/css-gradient-line
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::FloatWriter;

    fn assert_approx_eq(a: f32, b: f32, site: &str) {
        let mut a_str = String::new();
        let mut b_str = String::new();
        FloatWriter::write_float(&mut a_str, a).unwrap();
        FloatWriter::write_float(&mut b_str, b).unwrap();
        assert_eq!(a_str, b_str, "{site}");
    }

    fn assert_line(actual: (f32, f32, f32, f32), expected: (f32, f32, f32, f32)) {
        let (ax1, ay1, ax2, ay2) = actual;
        let (ex1, ey1, ex2, ey2) = expected;

        assert_approx_eq(ax1, ex1, "x1");
        assert_approx_eq(ay1, ey1, "y1");
        assert_approx_eq(ax2, ex2, "x2");
        assert_approx_eq(ay2, ey2, "y2");
    }

    #[test]
    fn converts_angle_to_line() {
        for (angle, line) in [
            (0.0, (0.5, 1.0, 0.5, 0.0)),
            (45.0, (0.0, 1.0, 1.0, 0.0)),
            (90.0, (0.0, 0.5, 1.0, 0.5)),
            (135.0, (0.0, 0.0, 1.0, 1.0)),
            (180.0, (0.5, 0.0, 0.5, 1.0)),
            (270.0, (1.0, 0.5, 0.0, 0.5)),
        ] {
            assert_line(angle_to_line(angle), line);
        }
    }
}
