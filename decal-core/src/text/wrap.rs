#[derive(Debug, Clone, Copy)]
pub enum TextWrap {
    None,
    Glyph,
    Word,
    /// wrap at the word level or fallback to glyph level if a word can't fit on
    /// a line by itself
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
