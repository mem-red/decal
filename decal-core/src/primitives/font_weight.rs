/// The weight of the font.
#[derive(Debug, Clone, Copy, Default)]
pub enum FontWeight {
    /// Thin weight (`100`).
    Thin,
    /// Extra-light weight (`200`).
    ExtraLight,
    /// Light weight (`300`).
    Light,
    /// Normal weight (`400`).
    #[default]
    Normal,
    /// Medium weight (`500`).
    Medium,
    /// Semi-bold weight (`600`).
    SemiBold,
    /// Bold weight (`700`).
    Bold,
    /// Extra-bold weight (`800`).
    ExtraBold,
    /// Black weight (`900`).
    Black,
    /// A custom font weight.
    Value(u16),
}

impl From<FontWeight> for cosmic_text::Weight {
    fn from(value: FontWeight) -> Self {
        match value {
            FontWeight::Thin => cosmic_text::Weight::THIN,
            FontWeight::ExtraLight => cosmic_text::Weight::EXTRA_LIGHT,
            FontWeight::Light => cosmic_text::Weight::LIGHT,
            FontWeight::Normal => cosmic_text::Weight::NORMAL,
            FontWeight::Medium => cosmic_text::Weight::MEDIUM,
            FontWeight::SemiBold => cosmic_text::Weight::SEMIBOLD,
            FontWeight::Bold => cosmic_text::Weight::BOLD,
            FontWeight::ExtraBold => cosmic_text::Weight::EXTRA_BOLD,
            FontWeight::Black => cosmic_text::Weight::BLACK,
            FontWeight::Value(value) => cosmic_text::Weight(value),
        }
    }
}
