use crate::macros::ff32;
use crate::utils::FloatWriter;
use std::fmt::{Display, Formatter, Write};
use strict_num::FiniteF32;
use taffy::prelude::{TaffyAuto, TaffyZero};

// do not expose this enum in public API to avoid mess
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
enum LengthInner {
    #[default]
    Zero,
    Auto,
    Absolute(FiniteF32),
    Percent(FiniteF32),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
pub struct Length<const AUTO: bool = true, const PERCENT: bool = true>(LengthInner);

impl<const PERCENT: bool> Length<true, PERCENT> {
    #[must_use]
    pub const fn auto() -> Self {
        Self(LengthInner::Auto)
    }
}

impl<const AUTO: bool> Length<AUTO, true> {
    #[must_use]
    pub fn percent(value: f32) -> Self {
        Self(LengthInner::Percent(ff32!(value / 100.0)))
    }

    #[must_use]
    pub fn percent_normalized(value: f32) -> Self {
        Self(LengthInner::Percent(ff32!(value)))
    }
}

impl<const AUTO: bool, const PERCENT: bool> Length<AUTO, PERCENT> {
    #[must_use]
    pub const fn zero() -> Self {
        Self(LengthInner::Zero)
    }

    #[must_use]
    pub fn units(value: f32) -> Self {
        Self(LengthInner::Absolute(ff32!(value)))
    }

    pub(crate) fn is_zero(&self) -> bool {
        match self.0 {
            LengthInner::Zero => true,
            LengthInner::Absolute(value) | LengthInner::Percent(value) => value.get() == 0.0,
            _ => false,
        }
    }

    pub(crate) fn resolve_abs(&self, full: f32) -> Option<f32> {
        match self.0 {
            LengthInner::Absolute(value) => Some(value.get()),
            LengthInner::Percent(value) => Some(value.get() * full),
            _ => None,
        }
    }
}

impl<const AUTO: bool, const PERCENT: bool> Display for Length<AUTO, PERCENT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            LengthInner::Zero => f.write_char('0'),
            LengthInner::Auto => f.write_str("auto"),
            LengthInner::Absolute(value) => f.write_float(value.get()),
            LengthInner::Percent(value) => {
                f.write_float(value.get() * 100.0)?;
                f.write_char('%')
            }
        }
    }
}

impl<const PERCENT: bool> Into<taffy::LengthPercentage> for Length<false, PERCENT> {
    fn into(self) -> taffy::LengthPercentage {
        match self.0 {
            LengthInner::Zero => taffy::LengthPercentage::ZERO,
            LengthInner::Absolute(value) => taffy::LengthPercentage::length(value.get()),
            LengthInner::Percent(value) => taffy::LengthPercentage::percent(value.get()),
            LengthInner::Auto => unreachable!(),
        }
    }
}

impl<const AUTO: bool, const PERCENT: bool> Into<taffy::LengthPercentageAuto>
    for Length<AUTO, PERCENT>
{
    fn into(self) -> taffy::LengthPercentageAuto {
        match self.0 {
            LengthInner::Auto => taffy::LengthPercentageAuto::AUTO,
            LengthInner::Zero => taffy::LengthPercentageAuto::ZERO,
            LengthInner::Absolute(value) => taffy::LengthPercentageAuto::length(value.get()),
            LengthInner::Percent(value) => taffy::LengthPercentageAuto::percent(value.get()),
        }
    }
}

impl<const AUTO: bool, const PERCENT: bool> Into<taffy::Dimension> for Length<AUTO, PERCENT> {
    fn into(self) -> taffy::Dimension {
        let length: taffy::LengthPercentageAuto = self.into();
        taffy::Dimension::from(length)
    }
}

//

pub trait IntoOptionalLength<const AUTO: bool = true, const PERCENT: bool = true> {
    fn into_optional_length(self) -> Option<Length<AUTO, PERCENT>>;
}

impl<const AUTO: bool, const PERCENT: bool> IntoOptionalLength<AUTO, PERCENT>
    for Option<Length<AUTO, PERCENT>>
{
    fn into_optional_length(self) -> Option<Length<AUTO, PERCENT>> {
        self
    }
}

impl<const AUTO: bool, const PERCENT: bool, T> IntoOptionalLength<AUTO, PERCENT> for T
where
    T: Into<Length<AUTO, PERCENT>> + Copy,
{
    fn into_optional_length(self) -> Option<Length<AUTO, PERCENT>> {
        Some(self.into())
    }
}

macro_rules! impl_into_unit_length {
    ($($dtype:ty),*) => {
        $(impl<const AUTO: bool, const PERCENT: bool> From<$dtype> for Length<AUTO, PERCENT> {
            fn from(value: $dtype) -> Self {
                Self::units(value as f32)
            }
        })*
    };
}

impl_into_unit_length!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);

//

pub(super) mod helpers {
    use super::Length;

    #[must_use]
    pub const fn zero<const AUTO: bool, const PERCENT: bool>() -> Length<AUTO, PERCENT> {
        Length::zero()
    }

    #[must_use]
    pub const fn auto<const PERCENT: bool>() -> Length<true, PERCENT> {
        Length::auto()
    }

    #[must_use]
    pub fn units<T, const AUTO: bool, const PERCENT: bool>(value: T) -> Length<AUTO, PERCENT>
    where
        T: Into<f64>,
    {
        Length::units(value.into() as f32)
    }

    #[must_use]
    pub fn pct<T, const AUTO: bool>(value: T) -> Length<AUTO, true>
    where
        T: Into<f64>,
    {
        Length::percent(value.into() as f32)
    }

    pub trait LengthExtension<const AUTO: bool, const PERCENT: bool>: Sized + Copy {
        #[must_use]
        fn units(self) -> Length<AUTO, PERCENT>;

        #[must_use]
        fn pct(self) -> Length<AUTO, true>;
    }

    macro_rules! impl_length_ext {
        ($($dtype:ty),*) => {
            $(impl<const AUTO: bool, const PERCENT: bool> LengthExtension<AUTO, PERCENT> for $dtype {
                fn units(self) -> Length<AUTO, PERCENT> {
                    Length::units(self as f32)
                }
                fn pct(self) -> Length<AUTO, true> {
                    Length::percent(self as f32)
                }
            })*
        };
    }

    impl_length_ext!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
}
