use taffy::{LengthPercentage, prelude::TaffyZero};

#[derive(Debug, Clone, Copy, Default)]
pub enum Length {
    #[default]
    Zero,
    Pixels(f32),
    Percent(f32),
}

impl Length {
    pub(crate) fn to_length_percentage(&self) -> LengthPercentage {
        match *self {
            Self::Zero => LengthPercentage::ZERO,
            Self::Pixels(value) => LengthPercentage::length(value),
            Self::Percent(value) => LengthPercentage::percent(value),
        }
    }
}
