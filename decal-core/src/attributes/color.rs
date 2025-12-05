use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: f32,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r,
            g,
            b,
            a: (a / 100.0).clamp(0.0, 1.0),
        }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 1.0 }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("rgba({},{},{},{})", self.r, self.g, self.b, self.a)
        )
    }
}
