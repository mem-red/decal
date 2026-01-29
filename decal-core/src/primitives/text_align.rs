/// The text alignment property.
#[derive(Debug, Clone, Copy)]
pub enum TextAlign {
    /// Align text to the start of the line.
    Left,
    /// Align text to the end of the line.
    Right,
    /// Center text within the available line width.
    Center,
    /// Distribute spacing so that each line (except the last) fills the
    /// available width.
    Justified,
    /// Align text to the logical end of the line.
    End,
}

impl From<TextAlign> for cosmic_text::Align {
    fn from(value: TextAlign) -> Self {
        match value {
            TextAlign::Left => cosmic_text::Align::Left,
            TextAlign::Right => cosmic_text::Align::Right,
            TextAlign::Center => cosmic_text::Align::Center,
            TextAlign::Justified => cosmic_text::Align::Justified,
            TextAlign::End => cosmic_text::Align::End,
        }
    }
}
