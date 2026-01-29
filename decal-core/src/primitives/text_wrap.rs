/// The text wrap property.
#[derive(Debug, Clone, Copy)]
pub enum TextWrap {
    /// Disables wrapping entirely.
    None,
    /// Allows wrapping at individual glyph boundaries.
    Glyph,
    /// Allows wrapping only at word boundaries.
    Word,
    /// Wraps at word boundaries, but falls back to glyph-level wrapping if a
    /// word cannot fit on a line by itself.
    WordOrGlyph,
}

impl From<TextWrap> for cosmic_text::Wrap {
    fn from(value: TextWrap) -> Self {
        match value {
            TextWrap::None => cosmic_text::Wrap::None,
            TextWrap::Glyph => cosmic_text::Wrap::Glyph,
            TextWrap::Word => cosmic_text::Wrap::Word,
            TextWrap::WordOrGlyph => cosmic_text::Wrap::WordOrGlyph,
        }
    }
}
