use super::Drawable;
use crate::{
    primitives::Paint,
    text::{
        FontStyle,
        FontWeight,
        TextAlign,
        TextWrap,
    },
};

/// Capability for configuring text appearance and typography on a node.
pub trait Textual: Drawable {
    /// Sets the text color.
    ///
    /// # Arguments
    /// - `value`: The [`Paint`] value applied to the text.
    ///
    /// # Returns
    /// - [`Self`]
    fn color<T>(mut self, value: T) -> Self
    where
        T: Into<Paint>,
    {
        let color = value.into();
        self.typography_mut().color = Some(color.clone());
        self.add_resources(color);
        self
    }

    /// Sets the font family of text.
    ///
    /// # Arguments
    /// - `family`: The font family name.
    ///
    /// # Returns
    /// - [`Self`]
    fn font_family<T, S>(mut self, family: T) -> Self
    where
        T: Into<Option<S>>,
        S: Into<String>,
    {
        self.typography_mut().family = family.into().map(Into::into);
        self
    }

    /// Sets the font size of text.
    ///
    /// # Arguments
    /// - `font_size`: The font size.
    ///
    /// # Returns
    /// - [`Self`]
    fn font_size<T>(mut self, font_size: T) -> Self
    where
        T: Into<Option<f32>>,
    {
        self.typography_mut().size = font_size.into();
        self
    }

    /// Sets the line height of text.
    ///
    /// # Arguments
    /// - `line_height`: The line height.
    ///
    /// # Returns
    /// - [`Self`]
    fn line_height<T>(mut self, line_height: T) -> Self
    where
        T: Into<Option<f32>>,
    {
        self.typography_mut().line_height = line_height.into();
        self
    }

    /// Sets the font weight of text.
    ///
    /// # Arguments
    /// - `font_weight`: The [`FontWeight`] to apply.
    ///
    /// # Returns
    /// - [`Self`]
    fn font_weight<T>(mut self, font_weight: T) -> Self
    where
        T: Into<Option<FontWeight>>,
    {
        self.typography_mut().weight = font_weight.into();
        self
    }

    /// Sets the letter spacing applied between text glyphs.
    ///
    /// # Arguments
    /// - `letter_spacing`: The spacing between characters.
    ///
    /// # Returns
    /// - [`Self`]
    fn letter_spacing<T>(mut self, letter_spacing: T) -> Self
    where
        T: Into<Option<f32>>,
    {
        self.typography_mut().letter_spacing = letter_spacing.into();
        self
    }

    /// Sets the font style of text.
    ///
    /// # Arguments
    /// - `font_style`: The [`FontStyle`] to apply.
    ///
    /// # Returns
    /// - [`Self`]
    fn font_style<T>(mut self, font_style: T) -> Self
    where
        T: Into<Option<FontStyle>>,
    {
        self.typography_mut().style = font_style.into();
        self
    }

    /// Sets the horizontal alignment of text within its container.
    ///
    /// # Arguments
    /// - `text_align`: The [`TextAlign`] behavior to apply.
    ///
    /// # Returns
    /// - [`Self`]
    fn text_align<T>(mut self, text_align: T) -> Self
    where
        T: Into<Option<TextAlign>>,
    {
        self.typography_mut().align = text_align.into();
        self
    }

    /// Sets the text wrapping behavior.
    ///
    /// # Arguments
    /// - `text_wrap`: The [`TextWrap`] behavior to apply.
    ///
    /// # Returns
    /// - [`Self`]
    fn text_wrap<T>(mut self, text_wrap: T) -> Self
    where
        T: Into<Option<TextWrap>>,
    {
        self.typography_mut().wrap = text_wrap.into();
        self
    }
}
