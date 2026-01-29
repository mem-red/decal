use crate::{
    macros::ff32,
    utils::FloatWriter,
};
use std::{
    fmt::{
        Display,
        Formatter,
        Write,
    },
    ops::{
        Add,
        Sub,
    },
};
use strict_num::FiniteF32;
use taffy::prelude::{
    TaffyAuto,
    TaffyZero,
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
enum LengthInner {
    #[default]
    Zero,
    Auto,
    Absolute(FiniteF32),
    Percent(FiniteF32),
}

impl LengthInner {
    fn negate(self) -> Self {
        match self {
            LengthInner::Zero => self,
            LengthInner::Auto => self,
            LengthInner::Absolute(x) => Self::Absolute(ff32!(-x.get())),
            LengthInner::Percent(x) => Self::Percent(ff32!(-x.get())),
        }
    }
}

/// The length value used for sizing, positioning, and spacing.
///
/// A [`Length`] can represent:
/// - an absolute value in layout units
/// - a percentage of the available space
/// - auto
/// - zero
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
pub struct Length<const AUTO: bool = true, const PERCENT: bool = true>(LengthInner);

impl<const PERCENT: bool> Length<true, PERCENT> {
    /// Creates a [`Length`] with the value `auto`.
    ///
    /// # Returns
    /// - [`Self`]
    #[must_use]
    pub const fn auto() -> Self {
        Self(LengthInner::Auto)
    }
}

impl<const AUTO: bool> Length<AUTO, true> {
    /// Creates a percentage [`Length`]. The value is interpreted as a
    /// percentage (e.g. `50.0` as `50%`).
    ///
    /// # Arguments
    /// - `value`: The percentage value.
    ///
    /// # Returns
    /// - [`Self`]
    #[must_use]
    pub fn percent(value: f32) -> Self {
        Self(LengthInner::Percent(ff32!(value / 100.0)))
    }

    /// Creates a normalized percentage [`Length`]. The value is interpreted as
    /// a normalized fraction (e.g. `0.5` as `50%`).
    ///
    /// # Arguments
    /// - `value`: The normalized percentage value.
    ///
    /// # Returns
    /// - [`Self`]
    #[must_use]
    pub fn percent_normalized(value: f32) -> Self {
        Self(LengthInner::Percent(ff32!(value)))
    }
}

impl<const AUTO: bool, const PERCENT: bool> Length<AUTO, PERCENT> {
    /// Creates a [`Length`] with zero as value.
    ///
    /// # Returns
    /// - [`Self`]
    #[must_use]
    pub const fn zero() -> Self {
        Self(LengthInner::Zero)
    }

    /// Creates an absolute [`Length`] value.
    ///
    /// # Arguments
    /// - `value`: The absolute length in layout units.
    ///
    /// # Returns
    /// - [`Self`]
    #[must_use]
    pub fn units(value: f32) -> Self {
        Self(LengthInner::Absolute(ff32!(value)))
    }

    /// Returns `true` if the length resolves to zero.
    pub(crate) fn is_zero(&self) -> bool {
        match self.0 {
            LengthInner::Zero => true,
            LengthInner::Absolute(x) | LengthInner::Percent(x) => x.get() == 0.0,
            _ => false,
        }
    }

    /// Tries to resolve the length into an absolute value.
    ///
    /// - Absolute values are returned as it is.
    /// - Percentage values are resolved relative to `full`.
    /// - Returns `None` otherwise.
    pub(crate) fn resolve_abs(&self, full: f32) -> Option<f32> {
        match self.0 {
            LengthInner::Absolute(value) => Some(value.get()),
            LengthInner::Percent(value) => Some(value.get() * full),
            _ => None,
        }
    }
}

impl Add for Length<false, false> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.0, rhs.0) {
            (LengthInner::Zero, b) => Self(b),
            (a, LengthInner::Zero) => Self(a),
            (LengthInner::Absolute(a), LengthInner::Absolute(b)) => Self::units(a.get() + b.get()),
            _ => unreachable!(),
        }
    }
}

impl Sub for Length<false, false> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.0, rhs.0) {
            (LengthInner::Zero, b) => Self(b.negate()),
            (a, LengthInner::Zero) => Self(a),
            (LengthInner::Absolute(a), LengthInner::Absolute(b)) => Self::units(a.get() - b.get()),
            _ => unreachable!(),
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

impl<const PERCENT: bool> From<Length<false, PERCENT>> for taffy::LengthPercentage {
    fn from(value: Length<false, PERCENT>) -> Self {
        match value.0 {
            LengthInner::Zero => taffy::LengthPercentage::ZERO,
            LengthInner::Absolute(value) => taffy::LengthPercentage::length(value.get()),
            LengthInner::Percent(value) => taffy::LengthPercentage::percent(value.get()),
            LengthInner::Auto => unreachable!(),
        }
    }
}

impl<const AUTO: bool, const PERCENT: bool> From<Length<AUTO, PERCENT>>
    for taffy::LengthPercentageAuto
{
    fn from(value: Length<AUTO, PERCENT>) -> Self {
        match value.0 {
            LengthInner::Auto => taffy::LengthPercentageAuto::AUTO,
            LengthInner::Zero => taffy::LengthPercentageAuto::ZERO,
            LengthInner::Absolute(value) => taffy::LengthPercentageAuto::length(value.get()),
            LengthInner::Percent(value) => taffy::LengthPercentageAuto::percent(value.get()),
        }
    }
}

impl<const AUTO: bool, const PERCENT: bool> From<Length<AUTO, PERCENT>> for taffy::Dimension {
    fn from(value: Length<AUTO, PERCENT>) -> Self {
        let length: taffy::LengthPercentageAuto = value.into();
        taffy::Dimension::from(length)
    }
}

/// Conversion into an optional [`Length`] value.
///
/// This is primarily used by builder APIs to allow both direct values and
/// `Option` values to be passed ergonomically.
pub trait IntoOptionalLength<const AUTO: bool = true, const PERCENT: bool = true> {
    /// Converts the value into an optional length.
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

    /// Returns a zero [`Length`] value.
    ///
    /// # Returns
    /// - [`Length`]
    #[must_use]
    pub const fn zero<const AUTO: bool, const PERCENT: bool>() -> Length<AUTO, PERCENT> {
        Length::zero()
    }

    /// Returns an `auto` [`Length`] value.
    ///
    /// # Returns
    /// - [`Length`]
    #[must_use]
    pub const fn auto<const PERCENT: bool>() -> Length<true, PERCENT> {
        Length::auto()
    }

    /// Creates an absolute [`Length`] value.
    ///
    /// # Arguments
    /// - `value`: The absolute length in layout units.
    ///
    /// # Returns
    /// - [`Length`]
    #[must_use]
    pub fn units<T, const AUTO: bool, const PERCENT: bool>(value: T) -> Length<AUTO, PERCENT>
    where
        T: Into<f64>,
    {
        Length::units(value.into() as f32)
    }

    /// Creates a percentage [`Length`] value. The value is interpreted as a
    /// percentage (e.g. `50.0` as `50%`).
    ///
    /// # Arguments
    /// - `value`: The percentage value.
    ///
    /// # Returns
    /// - [`Length`]
    #[must_use]
    pub fn pct<T, const AUTO: bool>(value: T) -> Length<AUTO, true>
    where
        T: Into<f64>,
    {
        Length::percent(value.into() as f32)
    }

    pub trait LengthExtension<const AUTO: bool, const PERCENT: bool>: Sized + Copy {
        /// Converts the value into an absolute [`Length`] value.
        ///
        /// # Returns
        /// - [`Length`]
        #[must_use]
        fn units(self) -> Length<AUTO, PERCENT>;

        /// Converts the value into a percentage [`Length`] value.
        ///
        /// # Returns
        /// - [`Length`]
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

#[cfg(test)]
mod tests {
    use super::{
        helpers::*,
        *,
    };

    type Len = Length<true, true>;
    type LenNoAutoNoPct = Length<false, false>;

    #[test]
    fn units_constructor() {
        let x = Len::units(5.5);
        assert_eq!(x, Length(LengthInner::Absolute(ff32!(5.5))));
        assert_eq!(x.resolve_abs(100.0), Some(5.5));
        assert!(!x.is_zero());
    }

    #[test]
    fn percent_constructor() {
        let x = Len::percent(50.0);
        assert_eq!(x, Length(LengthInner::Percent(ff32!(0.5))));
        assert_eq!(x.resolve_abs(200.0), Some(100.0));
        assert!(!x.is_zero());
    }

    #[test]
    fn percent_normalized_constructor() {
        assert_eq!(
            Len::percent_normalized(0.25).resolve_abs(400.0),
            Some(100.0)
        );
    }

    #[test]
    fn is_zero() {
        assert!(Len::zero().is_zero());
        assert!(Len::units(0.0).is_zero());
        assert!(Len::percent(0.0).is_zero());
    }

    #[test]
    fn resolves_absolute() {
        assert_eq!(Len::units(50.0).resolve_abs(100.0), Some(50.0));
        assert_eq!(Len::percent(25.0).resolve_abs(400.0), Some(100.0));
    }

    #[test]
    fn adds_zero() {
        let a = LenNoAutoNoPct::units(10.0);
        let b = LenNoAutoNoPct::zero();
        assert_eq!(a + b, a);
        assert_eq!(b + a, a);
    }

    #[test]
    fn adds_absolute() {
        let a = LenNoAutoNoPct::units(10.0);
        let b = LenNoAutoNoPct::units(5.0);
        assert_eq!(a + b, LenNoAutoNoPct::units(15.0));
    }

    #[test]
    fn subs_zero() {
        let a = LenNoAutoNoPct::units(10.0);
        let b = LenNoAutoNoPct::zero();
        assert_eq!(a - b, a);
    }

    #[test]
    fn subs_absolute() {
        let a = LenNoAutoNoPct::units(6.0);
        let b = LenNoAutoNoPct::units(4.0);
        assert_eq!(a - b, LenNoAutoNoPct::units(2.0));
    }

    #[test]
    fn sub_from_zero_negates() {
        let a = LenNoAutoNoPct::zero();
        let b = LenNoAutoNoPct::units(5.0);
        assert_eq!(a - b, LenNoAutoNoPct::units(-5.0));
    }

    #[test]
    fn renders_zero() {
        assert_eq!(Len::zero().to_string(), "0");
    }

    #[test]
    fn renders_auto() {
        assert_eq!(Len::auto().to_string(), "auto");
    }

    #[test]
    fn renders_units() {
        assert_eq!(Len::units(7.5).to_string(), "7.5");
    }

    #[test]
    fn renders_percent() {
        assert_eq!(Len::percent(50.0).to_string(), "50%");
    }

    #[test]
    fn units_from_integer() {
        assert_eq!(Len::from(10_u32), Len::units(10.0));
    }

    #[test]
    fn units_from_float() {
        assert_eq!(Len::from(2.5_f32), Len::units(2.5));
    }

    // helpers

    #[test]
    fn from_zero_helper() {
        assert!(zero::<true, true>().is_zero());
    }

    #[test]
    fn from_auto_helper() {
        assert_eq!(auto::<true>(), Length::auto());
    }

    #[test]
    fn from_units_helper() {
        assert_eq!(units::<_, true, true>(5), Len::units(5.0));
    }

    #[test]
    fn from_percent_helper() {
        assert_eq!(pct::<_, true>(50), Len::percent(50.0));
    }
}
