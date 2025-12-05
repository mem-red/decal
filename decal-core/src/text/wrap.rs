#[derive(Debug, Clone, Copy)]
pub enum TextWrap {
    None,
    Glyph,
    Word,
    /// wrap at the word level or fallback to glyph level if a word can't fit on a line by itself
    WordOrGlyph,
}

impl TextWrap {
    pub(crate) fn to_cosmic_wrap(&self) -> cosmic_text::Wrap {
        match self {
            TextWrap::None => cosmic_text::Wrap::None,
            TextWrap::Glyph => cosmic_text::Wrap::Glyph,
            TextWrap::Word => cosmic_text::Wrap::Word,
            TextWrap::WordOrGlyph => cosmic_text::Wrap::WordOrGlyph,
        }
    }
}
