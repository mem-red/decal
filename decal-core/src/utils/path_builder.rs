use core::fmt;
use ryu::Buffer;
use std::fmt::Write;

const SCALE: f32 = 1000.0;

#[derive(Debug)]
pub(crate) struct PathBuilder<'a, T>
where
    T: Write,
{
    out: &'a mut T,
    result: fmt::Result,
}

impl<'a, T> PathBuilder<'a, T>
where
    T: Write,
{
    pub(crate) fn new(out: &'a mut T) -> Self {
        Self {
            out,
            result: Ok(()),
        }
    }

    pub(crate) fn move_to(&mut self, x: f32, y: f32) -> &mut Self {
        self.write("M");
        self.write_float(x);
        self.write(" ");
        self.write_float(y);
        self.write(" ")
    }

    pub(crate) fn line_to(&mut self, x: f32, y: f32) -> &mut Self {
        self.write("L");
        self.write_float(x);
        self.write(" ");
        self.write_float(y);
        self.write(" ")
    }

    pub(crate) fn horizontal_to(&mut self, x: f32) -> &mut Self {
        self.write("H");
        self.write_float(x);
        self.write(" ")
    }

    pub(crate) fn vertical_to(&mut self, y: f32) -> &mut Self {
        self.write("V");
        self.write_float(y);
        self.write(" ")
    }

    pub(crate) fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) -> &mut Self {
        self.write("Q");
        self.write_float(cx);
        self.write(" ");
        self.write_float(cy);
        self.write(" ");
        self.write_float(x);
        self.write(" ");
        self.write_float(y);
        self.write(" ")
    }

    pub(crate) fn curve_to(
        &mut self,
        cx1: f32,
        cy1: f32,
        cx2: f32,
        cy2: f32,
        x: f32,
        y: f32,
    ) -> &mut Self {
        self.write("C");
        self.write_float(cx1);
        self.write(" ");
        self.write_float(cy1);
        self.write(" ");
        self.write_float(cx2);
        self.write(" ");
        self.write_float(cy2);
        self.write(" ");
        self.write_float(x);
        self.write(" ");
        self.write_float(y);
        self.write(" ")
    }

    pub(crate) fn arc_to(&mut self, rx: f32, ry: f32, x: f32, y: f32) -> &mut Self {
        self.write("A");
        self.write_float(rx);
        self.write(" ");
        self.write_float(ry);
        self.write(" 0 0 1 ");
        self.write_float(x);
        self.write(" ");
        self.write_float(y);
        self.write(" ")
    }

    pub(crate) fn close(&mut self) -> fmt::Result {
        self.write("Z").result
    }

    #[inline(always)]
    fn write(&mut self, str: &str) -> &mut Self {
        if self.result.is_ok() {
            self.result = self.out.write_str(str);
        }

        self
    }

    #[inline(always)]
    fn write_float(&mut self, mut value: f32) {
        value = (value * SCALE).round() / SCALE;
        let mut buf = Buffer::new();
        self.write(buf.format_finite(value));
    }
}
