use super::Drawable;
use crate::attributes::IntoPaint;
use crate::text::{FontStyle, FontWeight, TextAlign, TextWrap};

pub trait Textual: Drawable {
    fn color<T>(mut self, value: T) -> Self
    where
        T: IntoPaint,
    {
        let color = value.into_paint();
        self.typography_mut().color = Some(color.clone());
        self.add_resources(color);

        self
    }

    fn font_family<T, S>(mut self, family: T) -> Self
    where
        T: Into<Option<S>>,
        S: Into<String>,
    {
        self.typography_mut().family = family.into().map(|s| s.into());
        self
    }

    fn font_size<T, F>(mut self, font_size: T) -> Self
    where
        T: Into<Option<F>>,
        F: Into<f64>,
    {
        self.typography_mut().size = font_size.into().map(|x| x.into() as f32);
        self
    }

    fn line_height<T, F>(mut self, line_height: T) -> Self
    where
        T: Into<Option<F>>,
        F: Into<f64>,
    {
        self.typography_mut().line_height = line_height.into().map(|x| x.into() as f32);
        self
    }

    fn font_weight<T>(mut self, font_weight: T) -> Self
    where
        T: Into<Option<FontWeight>>,
    {
        self.typography_mut().weight = font_weight.into();
        self
    }

    fn letter_spacing<T, F>(mut self, letter_spacing: T) -> Self
    where
        T: Into<Option<F>>,
        F: Into<f64>,
    {
        self.typography_mut().letter_spacing = letter_spacing.into().map(|x| x.into() as f32);
        self
    }

    fn font_style<T>(mut self, font_style: T) -> Self
    where
        T: Into<Option<FontStyle>>,
    {
        self.typography_mut().style = font_style.into();
        self
    }

    fn text_align<T>(mut self, text_align: T) -> Self
    where
        T: Into<Option<TextAlign>>,
    {
        self.typography_mut().align = text_align.into();
        self
    }

    fn text_wrap<T>(mut self, text_wrap: T) -> Self
    where
        T: Into<Option<TextWrap>>,
    {
        self.typography_mut().wrap = text_wrap.into();
        self
    }
}
