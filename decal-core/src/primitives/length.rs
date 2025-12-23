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
    Pixels(FiniteF32),
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
    pub fn percent<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        let value = value.into() as f32;
        Self(LengthInner::Percent(ff32!(value / 100.0)))
    }

    pub(crate) fn get_percent(&self) -> Option<f32> {
        match self.0 {
            LengthInner::Percent(pc) => Some(pc.get()),
            _ => None,
        }
    }
}

impl<const AUTO: bool, const PERCENT: bool> Length<AUTO, PERCENT> {
    #[must_use]
    pub const fn zero() -> Self {
        Self(LengthInner::Zero)
    }

    #[must_use]
    pub fn pixels<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self(LengthInner::Pixels(ff32!(value.into() as f32)))
    }

    pub(crate) fn is_zero(&self) -> bool {
        match self.0 {
            LengthInner::Zero => true,
            LengthInner::Pixels(px) => px.get() == 0.0,
            LengthInner::Percent(pc) => pc.get() == 0.0,
            _ => false,
        }
    }

    pub(crate) fn get_pixels(&self) -> Option<f32> {
        match self.0 {
            LengthInner::Pixels(px) => Some(px.get()),
            _ => None,
        }
    }

    pub(crate) fn resolve_abs(&self, full: f32) -> Option<f32> {
        match self.0 {
            LengthInner::Pixels(px) => Some(px.get()),
            LengthInner::Percent(pc) => Some(pc.get() * full),
            _ => None,
        }
    }
}

impl<const AUTO: bool, const PERCENT: bool> Display for Length<AUTO, PERCENT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            LengthInner::Zero => f.write_char('0'),
            LengthInner::Auto => f.write_str("auto"),
            LengthInner::Pixels(px) => f.write_fmt(format_args!("{px}")),
            LengthInner::Percent(pc) => {
                FloatWriter::write_float(f, pc.get())?;
                f.write_char('%')
            }
        }
    }
}

impl<const PERCENT: bool> Into<taffy::LengthPercentage> for Length<false, PERCENT> {
    fn into(self) -> taffy::LengthPercentage {
        match self.0 {
            LengthInner::Zero => taffy::LengthPercentage::ZERO,
            LengthInner::Pixels(value) => taffy::LengthPercentage::length(value.get()),
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
            LengthInner::Pixels(value) => taffy::LengthPercentageAuto::length(value.get()),
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
    pub fn px<T, const AUTO: bool, const PERCENT: bool>(value: T) -> Length<AUTO, PERCENT>
    where
        T: Into<f64>,
    {
        Length::pixels(value)
    }

    #[must_use]
    pub fn pc<T, const AUTO: bool>(value: T) -> Length<AUTO, true>
    where
        T: Into<f64>,
    {
        Length::percent(value)
    }

    pub trait LengthExtension<const AUTO: bool, const PERCENT: bool>: Sized + Copy {
        #[must_use]
        fn px(self) -> Length<AUTO, PERCENT>;

        #[must_use]
        fn pc(self) -> Length<AUTO, true>;
    }

    macro_rules! impl_length_ext {
        ($($dtype:ty),*) => {
            $(impl<const AUTO: bool, const PERCENT: bool> LengthExtension<AUTO, PERCENT> for $dtype {
                fn px(self) -> Length<AUTO, PERCENT> {
                    Length::pixels(self as f64)
                }

                fn pc(self) -> Length<AUTO, true> {
                    Length::percent(self as f64)
                }
            })*
        };
    }

    impl_length_ext!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
}
