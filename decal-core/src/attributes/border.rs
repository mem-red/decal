use crate::primitives::{
    Length,
    Rect,
};

/// Type alias representing the width of a border edge.
type BorderWidth = Length<false, true>;

/// Type alias for a rectangular border composed of four independent border
/// widths.
type Border = Rect<BorderWidth>;

/// Conversion trait for values that can be interpreted as a full rectangular
/// border.
pub trait IntoBorder {
    /// Converts the value into a rectangular border definition.
    ///
    /// This abstraction allows a wide range of ergonomic inputs while keeping
    /// call sites concise.
    ///
    /// # Returns
    /// - `Some(Border)` when the value can be expanded into a full border.
    /// - `None` when the value semantically represents the absence of a border.
    fn into_border(self) -> Option<Border>;
}

/// Identity conversion for an optional border.
impl IntoBorder for Option<Border> {
    /// Returns the contained border without modification.
    #[inline]
    fn into_border(self) -> Option<Border> {
        self
    }
}

/// Wraps an existing border into an optional value.
impl IntoBorder for Border {
    /// Converts a concrete border into `Some(Border)`.
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(self)
    }
}

/// Expands a single value into a uniform border applied to all sides.
impl<T> IntoBorder for T
where
    T: Into<BorderWidth> + Copy,
{
    /// Uses the same width for the top, right, bottom, and left edges.
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(Rect {
            top: self.into(),
            right: self.into(),
            bottom: self.into(),
            left: self.into(),
        })
    }
}

/// Converts a two-value tuple into a vertical and horizontal border.
impl<T> IntoBorder for (T, T)
where
    T: Into<BorderWidth> + Copy,
{
    /// Interprets the tuple as `(vertical, horizontal)`.
    ///
    /// The first value is applied to top and bottom, the second to left and
    /// right.
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(Rect {
            top: self.0.into(),
            right: self.1.into(),
            bottom: self.0.into(),
            left: self.1.into(),
        })
    }
}

/// Converts a three-value tuple into a border with an inferred left edge.
impl<T> IntoBorder for (T, T, T)
where
    T: Into<BorderWidth> + Copy,
{
    /// Interprets the tuple as `(top, horizontal, bottom)`.
    ///
    /// The left edge reuses the horizontal value.
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(Rect {
            top: self.0.into(),
            right: self.1.into(),
            bottom: self.2.into(),
            left: self.1.into(),
        })
    }
}

/// Converts a four-value tuple into an explicit border for all edges.
impl<T> IntoBorder for (T, T, T, T)
where
    T: Into<BorderWidth> + Copy,
{
    /// Interprets the tuple as `(top, right, bottom, left)` in clockwise order.
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(Rect {
            top: self.0.into(),
            right: self.1.into(),
            bottom: self.2.into(),
            left: self.3.into(),
        })
    }
}

/// Array-based shorthand for two-value border definitions.
impl<T> IntoBorder for [T; 2]
where
    T: Into<BorderWidth> + Copy,
{
    /// Delegates to the two-element tuple implementation.
    #[inline]
    fn into_border(self) -> Option<Border> {
        IntoBorder::into_border((self[0], self[1]))
    }
}

/// Array-based shorthand for three-value border definitions.
impl<T> IntoBorder for [T; 3]
where
    T: Into<BorderWidth> + Copy,
{
    /// Delegates to the three-element tuple implementation.
    #[inline]
    fn into_border(self) -> Option<Border> {
        IntoBorder::into_border((self[0], self[1], self[2]))
    }
}

/// Array-based shorthand for four-value border definitions.
impl<T> IntoBorder for [T; 4]
where
    T: Into<BorderWidth> + Copy,
{
    /// Delegates to the four-element tuple implementation.
    #[inline]
    fn into_border(self) -> Option<Border> {
        IntoBorder::into_border((self[0], self[1], self[2], self[3]))
    }
}

/// Type alias representing a pair of border widths, typically horizontal and
/// vertical.
type BorderPair = (BorderWidth, BorderWidth);

/// Conversion trait for values that can be interpreted as a border width pair.
pub trait IntoBorderPair {
    /// Converts the value into a pair of border widths.
    ///
    /// This is commonly used where only two orthogonal dimensions are required.
    ///
    /// # Returns
    /// - `Some((BorderWidth, BorderWidth))` when conversion succeeds.
    /// - `None` when the value represents no border pair.
    fn into_border_pair(self) -> Option<BorderPair>;
}

/// Expands a single value into a symmetric border pair.
impl<T> IntoBorderPair for T
where
    T: Into<BorderWidth> + Copy,
{
    /// Uses the same width for both elements of the pair.
    #[inline]
    fn into_border_pair(self) -> Option<BorderPair> {
        Some((self.into(), self.into()))
    }
}

/// Converts a two-value tuple into a border width pair.
impl<T> IntoBorderPair for (T, T)
where
    T: Into<BorderWidth> + Copy,
{
    /// Interprets the tuple elements directly without reordering.
    #[inline]
    fn into_border_pair(self) -> Option<BorderPair> {
        Some((self.0.into(), self.1.into()))
    }
}

/// Array-based shorthand for border width pairs.
impl<T> IntoBorderPair for [T; 2]
where
    T: Into<BorderWidth> + Copy,
{
    /// Delegates to the tuple-based border pair conversion.
    #[inline]
    fn into_border_pair(self) -> Option<BorderPair> {
        IntoBorderPair::into_border_pair((self[0], self[1]))
    }
}
