use crate::primitives::{
    Length,
    Size,
};

/// Type alias representing the size of a gap along a single axis.
type GapSize = Length<false, true>;

/// Type alias for a two-dimensional gap defined by horizontal and vertical
/// sizes.
type Gap = Size<GapSize>;

/// Conversion trait for values that can be interpreted as layout gaps.
pub trait IntoGap {
    /// Converts the value into a gap definition.
    ///
    /// This abstraction allows a wide range of ergonomic inputs while keeping
    /// call sites concise.
    ///
    /// # Returns
    /// - `Some(Gap)` when the value can be expanded into horizontal and
    ///   vertical gaps.
    /// - `None` when the value semantically represents the absence of a gap.
    fn into_gap(self) -> Option<Gap>;
}

/// Identity conversion for an optional gap.
impl IntoGap for Option<Gap> {
    /// Returns the contained gap without modification.
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        self
    }
}

/// Wraps an existing gap into an optional value.
impl IntoGap for Gap {
    /// Converts a concrete gap into `Some(Gap)`.
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        Some(self)
    }
}

/// Expands a single value into equal horizontal and vertical gaps.
impl<T> IntoGap for T
where
    T: Into<GapSize> + Copy,
{
    /// Uses the same size for both width and height gaps.
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        Some(Size {
            width: self.into(),
            height: self.into(),
        })
    }
}

/// Converts a two-value tuple into explicit horizontal and vertical gaps.
impl<T> IntoGap for (T, T)
where
    T: Into<GapSize> + Copy,
{
    /// Interprets the tuple as `(horizontal, vertical)` without reordering.
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        Some(Size {
            width: self.0.into(),
            height: self.1.into(),
        })
    }
}

/// Array-based shorthand for gap definitions.
impl<T> IntoGap for [T; 2]
where
    T: Into<GapSize> + Copy,
{
    /// Delegates to the two-element tuple implementation.
    #[inline]
    fn into_gap(self) -> Option<Gap> {
        IntoGap::into_gap((self[0], self[1]))
    }
}
