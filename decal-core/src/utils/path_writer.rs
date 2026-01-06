use super::writer::Writer;
use crate::utils::FloatWriter;
use std::fmt::Write;

#[derive(Debug)]
pub(crate) struct PathWriter<'a, T>(&'a mut T)
where
    T: Write;

impl<T> FloatWriter<T> for PathWriter<'_, T>
where
    T: Write,
{
    fn out_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Writer<T> for PathWriter<'_, T> where T: Write {}

impl<'a, T> PathWriter<'a, T>
where
    T: Write,
{
    pub(crate) fn new(out: &'a mut T) -> Self {
        Self(out)
    }

    pub(crate) fn move_to(&mut self, x: f32, y: f32) -> Result<&mut Self, std::fmt::Error> {
        self.char('M')?.float(x)?.space()?.float(y)?.space()
    }

    pub(crate) fn line_to(&mut self, x: f32, y: f32) -> Result<&mut Self, std::fmt::Error> {
        self.char('L')?.float(x)?.space()?.float(y)?.space()
    }

    pub(crate) fn horizontal_to(&mut self, x: f32) -> Result<&mut Self, std::fmt::Error> {
        self.char('H')?.float(x)?.space()
    }

    pub(crate) fn vertical_to(&mut self, y: f32) -> Result<&mut Self, std::fmt::Error> {
        self.char('V')?.float(y)?.space()
    }

    pub(crate) fn quad_to(
        &mut self,
        cx: f32,
        cy: f32,
        x: f32,
        y: f32,
    ) -> Result<&mut Self, std::fmt::Error> {
        self.char('Q')?
            .float(cx)?
            .space()?
            .float(cy)?
            .space()?
            .float(x)?
            .space()?
            .float(y)?
            .space()
    }

    pub(crate) fn curve_to(
        &mut self,
        cx1: f32,
        cy1: f32,
        cx2: f32,
        cy2: f32,
        x: f32,
        y: f32,
    ) -> Result<&mut Self, std::fmt::Error> {
        self.char('C')?
            .float(cx1)?
            .space()?
            .float(cy1)?
            .space()?
            .float(cx2)?
            .space()?
            .float(cy2)?
            .space()?
            .float(x)?
            .space()?
            .float(y)?
            .space()
    }

    pub(crate) fn arc_to(
        &mut self,
        rx: f32,
        ry: f32,
        x: f32,
        y: f32,
    ) -> Result<&mut Self, std::fmt::Error> {
        self.char('A')?
            .float(rx)?
            .space()?
            .float(ry)?
            .str(" 0 0 1 ")?
            .float(x)?
            .space()?
            .float(y)?
            .space()
    }

    pub(crate) fn close(&mut self) -> std::fmt::Result {
        self.char('Z').map(|_| ())
    }
}
