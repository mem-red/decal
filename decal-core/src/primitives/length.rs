use taffy::prelude::{TaffyAuto, TaffyZero};

#[derive(Debug, Clone, Copy, Default)]
enum LengthInner {
    #[default]
    Zero,
    Auto,
    Pixels(f32),
    Percent(f32),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Length(LengthInner);

impl Length {
    pub const fn zero() -> Self {
        Self(LengthInner::Zero)
    }

    pub const fn auto() -> Self {
        Self(LengthInner::Auto)
    }

    pub fn pixels<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self(LengthInner::Pixels(value.into() as f32))
    }

    pub fn percent<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self(LengthInner::Percent(value.into() as f32))
    }

    pub(crate) fn to_length_percentage(&self) -> taffy::LengthPercentage {
        match self.0 {
            LengthInner::Auto | LengthInner::Zero => taffy::LengthPercentage::ZERO,
            LengthInner::Pixels(value) => taffy::LengthPercentage::length(value),
            LengthInner::Percent(value) => taffy::LengthPercentage::percent(value / 100.0),
        }
    }

    pub(crate) fn to_length_percentage_auto(&self) -> taffy::LengthPercentageAuto {
        match self.0 {
            LengthInner::Auto => taffy::LengthPercentageAuto::AUTO,
            LengthInner::Zero => taffy::LengthPercentageAuto::ZERO,
            LengthInner::Pixels(value) => taffy::LengthPercentageAuto::length(value),
            LengthInner::Percent(value) => taffy::LengthPercentageAuto::percent(value / 100.0),
        }
    }
}

pub(super) mod helpers {
    use super::Length;

    pub const fn zero() -> Length {
        Length::zero()
    }

    pub const fn auto() -> Length {
        Length::auto()
    }

    pub fn pix<T>(value: T) -> Length
    where
        T: Into<f64>,
    {
        Length::pixels(value)
    }

    pub fn pct<T>(value: T) -> Length
    where
        T: Into<f64>,
    {
        Length::percent(value)
    }
}
