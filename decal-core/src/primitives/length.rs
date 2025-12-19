use taffy::prelude::{TaffyAuto, TaffyZero};

// do not expose this enum in public API to avoid mess
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
    #[must_use]
    pub const fn zero() -> Self {
        Self(LengthInner::Zero)
    }

    #[must_use]
    pub const fn auto() -> Self {
        Self(LengthInner::Auto)
    }

    #[must_use]
    pub fn pixels<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self(LengthInner::Pixels(value.into() as f32))
    }

    #[must_use]
    pub fn percent<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        let value = value.into() as f32;
        debug_assert!(value >= 0.0 && value <= 100.0);
        Self(LengthInner::Percent((value / 100.0).clamp(0.0, 1.0)))
    }

    pub(crate) fn get_pixels(&self) -> Option<f32> {
        match self.0 {
            LengthInner::Pixels(pix) => Some(pix),
            _ => None,
        }
    }

    pub(crate) fn get_percent(&self) -> Option<f32> {
        match self.0 {
            LengthInner::Percent(pct) => Some(pct),
            _ => None,
        }
    }

    pub(crate) fn resolve_abs(&self, full: f32) -> f32 {
        if let Some(pct) = self.get_percent() {
            pct * full
        } else {
            self.get_pixels().unwrap_or_default()
        }
    }
}

impl Into<taffy::LengthPercentage> for Length {
    fn into(self) -> taffy::LengthPercentage {
        match self.0 {
            LengthInner::Auto | LengthInner::Zero => taffy::LengthPercentage::ZERO,
            LengthInner::Pixels(value) => taffy::LengthPercentage::length(value),
            LengthInner::Percent(value) => taffy::LengthPercentage::percent(value),
        }
    }
}

impl Into<taffy::LengthPercentageAuto> for Length {
    fn into(self) -> taffy::LengthPercentageAuto {
        match self.0 {
            LengthInner::Auto => taffy::LengthPercentageAuto::AUTO,
            LengthInner::Zero => taffy::LengthPercentageAuto::ZERO,
            LengthInner::Pixels(value) => taffy::LengthPercentageAuto::length(value),
            LengthInner::Percent(value) => taffy::LengthPercentageAuto::percent(value),
        }
    }
}

impl Into<taffy::Dimension> for Length {
    fn into(self) -> taffy::Dimension {
        let length: taffy::LengthPercentageAuto = self.into();
        taffy::Dimension::from(length)
    }
}

pub(super) mod helpers {
    use super::Length;

    #[must_use]
    pub const fn zero() -> Length {
        Length::zero()
    }

    #[must_use]
    pub const fn auto() -> Length {
        Length::auto()
    }

    #[must_use]
    pub fn px<T>(value: T) -> Length
    where
        T: Into<f64>,
    {
        Length::pixels(value)
    }

    #[must_use]
    pub fn pc<T>(value: T) -> Length
    where
        T: Into<f64>,
    {
        Length::percent(value)
    }

    pub trait LengthExtension: Sized {
        #[must_use]
        fn px(self) -> Length;

        #[must_use]
        fn pc(self) -> Length;
    }

    macro_rules! impl_length_ext {
        ($($dtype:ty),*) => {
            $(impl LengthExtension for $dtype {
                fn px(self) -> Length {
                    Length::pixels(self as f64)
                }

                fn pc(self) -> Length {
                    Length::percent(self as f64)
                }
            })*
        };
    }

    impl_length_ext!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
}
