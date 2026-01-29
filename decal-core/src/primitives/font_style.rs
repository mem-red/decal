/// The font style property.
#[derive(Debug, Clone, Copy)]
pub enum FontStyle {
    /// The normal font style.
    Normal,
    /// The italic font style.
    Italic,
    /// The oblique font style.
    Oblique,
}

impl From<FontStyle> for cosmic_text::Style {
    fn from(value: FontStyle) -> Self {
        match value {
            FontStyle::Normal => cosmic_text::Style::Normal,
            FontStyle::Italic => cosmic_text::Style::Italic,
            FontStyle::Oblique => cosmic_text::Style::Oblique,
        }
    }
}
