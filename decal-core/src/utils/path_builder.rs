use core::fmt;
use std::fmt::Write;

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
        self.write(format_args!("M{x} {y} "))
    }

    pub(crate) fn line_to(&mut self, x: f32, y: f32) -> &mut Self {
        self.write(format_args!("L{x} {y} "))
    }

    pub(crate) fn horizontal_to(&mut self, x: f32) -> &mut Self {
        self.write(format_args!("H{x} "))
    }

    pub(crate) fn vertical_to(&mut self, y: f32) -> &mut Self {
        self.write(format_args!("V{y} "))
    }

    pub(crate) fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) -> &mut Self {
        self.write(format_args!("Q{cx} {cy} {x} {y} "))
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
        self.write(format_args!("C{cx1} {cy1} {cx2} {cy2} {x} {y} "))
    }

    pub(crate) fn arc_to(&mut self, rx: f32, ry: f32, x: f32, y: f32) -> &mut Self {
        self.write(format_args!("A{rx} {ry} 0 0 1 {x} {y} "))
    }

    pub(crate) fn close(&mut self) -> fmt::Result {
        self.write(format_args!("Z")).result
    }

    #[inline(always)]
    fn write(&mut self, args: fmt::Arguments) -> &mut Self {
        if self.result.is_ok() {
            self.result = self.out.write_fmt(args);
        }

        self
    }
}
