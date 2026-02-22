/// The text ellipsis behavior.
#[derive(Debug, Clone, Copy, Default)]
pub enum Ellipsize {
    /// No ellipsizing.
    #[default]
    None,
    /// Ellipsizes the start of the text after the specified number of lines.
    ///
    /// The value specifies the number of lines to show before ellipsizing the
    /// rest.
    Start(usize),
    /// Ellipsizes the middle of the text after the specified number of lines.
    ///
    /// The value specifies the number of lines to show before ellipsizing the
    /// rest.
    Middle(usize),
    /// Ellipsizes the end of the text after the specified number of lines.
    ///
    /// The value specifies the number of lines to show before ellipsizing the
    /// rest.
    End(usize),
}

impl From<Ellipsize> for cosmic_text::Ellipsize {
    fn from(value: Ellipsize) -> Self {
        match value {
            Ellipsize::None => cosmic_text::Ellipsize::None,
            Ellipsize::Start(lines) => {
                cosmic_text::Ellipsize::Start(cosmic_text::EllipsizeHeightLimit::Lines(lines))
            }
            Ellipsize::Middle(lines) => {
                cosmic_text::Ellipsize::Middle(cosmic_text::EllipsizeHeightLimit::Lines(lines))
            }
            Ellipsize::End(lines) => {
                cosmic_text::Ellipsize::End(cosmic_text::EllipsizeHeightLimit::Lines(lines))
            }
        }
    }
}
