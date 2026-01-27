use crate::primitives::{
    Length,
    Rect,
};

/// Type alias representing the size of padding on a single edge.
type PaddingValue = Length<false, true>;

/// Type alias for a rectangular padding composed of four independent padding
/// values.
type Padding = Rect<PaddingValue>;

/// Conversion trait for values that can be interpreted as rectangular padding.
pub trait IntoPadding {
    /// Converts the value into a padding definition.
    ///
    /// This abstraction allows a wide range of ergonomic inputs while keeping
    /// call sites concise.
    ///
    /// # Returns
    /// - `Some(Padding)` when the value can be expanded into four padding
    ///   edges.
    /// - `None` when the value semantically represents the absence of padding.
    fn into_padding(self) -> Option<Padding>;
}

/// Identity conversion for an optional padding.
impl IntoPadding for Option<Padding> {
    /// Returns the contained padding without modification.
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        self
    }
}

/// Wraps an existing padding into an optional value.
impl IntoPadding for Padding {
    /// Converts a concrete padding into `Some(Padding)`.
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(self)
    }
}

/// Expands a single value into uniform padding applied to all edges.
impl<T> IntoPadding for T
where
    T: Into<PaddingValue> + Copy,
{
    /// Uses the same padding value for top, right, bottom, and left.
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(Rect {
            top: self.into(),
            right: self.into(),
            bottom: self.into(),
            left: self.into(),
        })
    }
}

/// Converts a two-value tuple into vertical and horizontal padding.
impl<T> IntoPadding for (T, T)
where
    T: Into<PaddingValue> + Copy,
{
    /// Interprets the tuple as `(vertical, horizontal)`.
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(Rect {
            top: self.0.into(),
            right: self.1.into(),
            bottom: self.0.into(),
            left: self.1.into(),
        })
    }
}

/// Converts a three-value tuple into padding with an inferred left edge.
impl<T> IntoPadding for (T, T, T)
where
    T: Into<PaddingValue> + Copy,
{
    /// Interprets the tuple as `(top, horizontal, bottom)`.
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(Rect {
            top: self.0.into(),
            right: self.1.into(),
            bottom: self.2.into(),
            left: self.1.into(),
        })
    }
}

/// Converts a four-value tuple into explicit padding for all edges.
impl<T> IntoPadding for (T, T, T, T)
where
    T: Into<PaddingValue> + Copy,
{
    /// Interprets the tuple as `(top, right, bottom, left)` in clockwise order.
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(Rect {
            top: self.0.into(),
            right: self.1.into(),
            bottom: self.2.into(),
            left: self.3.into(),
        })
    }
}

/// Array-based shorthand for two-value padding definitions.
impl<T> IntoPadding for [T; 2]
where
    T: Into<PaddingValue> + Copy,
{
    /// Delegates to the two-element tuple implementation.
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        IntoPadding::into_padding((self[0], self[1]))
    }
}

/// Array-based shorthand for three-value padding definitions.
impl<T> IntoPadding for [T; 3]
where
    T: Into<PaddingValue> + Copy,
{
    /// Delegates to the three-element tuple implementation.
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        IntoPadding::into_padding((self[0], self[1], self[2]))
    }
}

/// Array-based shorthand for four-value padding definitions.
impl<T> IntoPadding for [T; 4]
where
    T: Into<PaddingValue> + Copy,
{
    /// Delegates to the four-element tuple implementation.
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        IntoPadding::into_padding((self[0], self[1], self[2], self[3]))
    }
}

/// Type alias representing a pair of padding values, typically horizontal and
/// vertical.
type PaddingPair = (PaddingValue, PaddingValue);

/// Conversion trait for values that can be interpreted as a padding pair.
pub trait IntoPaddingPair {
    /// Converts the value into a pair of padding values.
    ///
    /// # Returns
    /// - `Some(PaddingPair)` when the value can be expanded into a pair.
    /// - `None` when the value semantically represents the absence of padding.
    fn into_padding_pair(self) -> Option<PaddingPair>;
}

/// Expands a single value into a symmetric padding pair.
impl<T> IntoPaddingPair for T
where
    T: Into<PaddingValue> + Copy,
{
    /// Uses the same padding value for both elements of the pair.
    #[inline]
    fn into_padding_pair(self) -> Option<PaddingPair> {
        Some((self.into(), self.into()))
    }
}

/// Converts a two-value tuple into a padding pair.
impl<T> IntoPaddingPair for (T, T)
where
    T: Into<PaddingValue> + Copy,
{
    /// Interprets the tuple elements directly without reordering.
    #[inline]
    fn into_padding_pair(self) -> Option<PaddingPair> {
        Some((self.0.into(), self.1.into()))
    }
}

/// Array-based shorthand for padding pair definitions.
impl<T> IntoPaddingPair for [T; 2]
where
    T: Into<PaddingValue> + Copy,
{
    /// Delegates to the tuple-based padding pair conversion.
    #[inline]
    fn into_padding_pair(self) -> Option<PaddingPair> {
        IntoPaddingPair::into_padding_pair((self[0], self[1]))
    }
}
