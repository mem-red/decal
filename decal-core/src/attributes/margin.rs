use crate::primitives::{
    Length,
    Rect,
};

/// Type alias representing margin values for all four edges.
type Margin = Rect<Length>;

/// Conversion trait for values that can be interpreted as a rectangular margin.
pub trait IntoMargin {
    /// Converts the value into a margin definition.
    ///
    ///
    /// This abstraction allows a wide range of ergonomic inputs while keeping
    /// call sites concise.
    ///
    /// # Returns
    /// - `Some(Margin)` when the value can be expanded into four margin edges.
    /// - `None` when the value semantically represents the absence of margin.
    fn into_margin(self) -> Option<Margin>;
}

/// Identity conversion for an optional margin.
impl IntoMargin for Option<Margin> {
    /// Returns the contained margin without modification.
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        self
    }
}

/// Wraps an existing margin into an optional value.
impl IntoMargin for Margin {
    /// Converts a concrete margin into `Some(Margin)`.
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(self)
    }
}

/// Expands a single value into a uniform margin applied to all edges.
impl<T> IntoMargin for T
where
    T: Into<Length> + Copy,
{
    /// Uses the same margin value for top, right, bottom, and left.
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(Rect {
            top: self.into(),
            right: self.into(),
            bottom: self.into(),
            left: self.into(),
        })
    }
}

/// Converts a two-value tuple into vertical and horizontal margins.
impl<T> IntoMargin for (T, T)
where
    T: Into<Length> + Copy,
{
    /// Interprets the tuple as `(vertical, horizontal)`.
    ///
    /// The first value is applied to top and bottom, the second to left and
    /// right.
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(Rect {
            top: self.0.into(),
            right: self.1.into(),
            bottom: self.0.into(),
            left: self.1.into(),
        })
    }
}

/// Converts a three-value tuple into a margin with an inferred left edge.
impl<T> IntoMargin for (T, T, T)
where
    T: Into<Length> + Copy,
{
    /// Interprets the tuple as `(top, horizontal, bottom)`.
    ///
    /// The left edge reuses the horizontal value.
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(Rect {
            top: self.0.into(),
            right: self.1.into(),
            bottom: self.2.into(),
            left: self.1.into(),
        })
    }
}

/// Converts a four-value tuple into an explicit margin for all edges.
impl<T> IntoMargin for (T, T, T, T)
where
    T: Into<Length> + Copy,
{
    /// Interprets the tuple as `(top, right, bottom, left)` in clockwise order.
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(Rect {
            top: self.0.into(),
            right: self.1.into(),
            bottom: self.2.into(),
            left: self.3.into(),
        })
    }
}

/// Array-based shorthand for two-value margin definitions.
impl<T> IntoMargin for [T; 2]
where
    T: Into<Length> + Copy,
{
    /// Delegates to the two-element tuple implementation.
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        IntoMargin::into_margin((self[0], self[1]))
    }
}

/// Array-based shorthand for three-value margin definitions.
impl<T> IntoMargin for [T; 3]
where
    T: Into<Length> + Copy,
{
    /// Delegates to the three-element tuple implementation.
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        IntoMargin::into_margin((self[0], self[1], self[2]))
    }
}

/// Array-based shorthand for four-value margin definitions.
impl<T> IntoMargin for [T; 4]
where
    T: Into<Length> + Copy,
{
    /// Delegates to the four-element tuple implementation.
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        IntoMargin::into_margin((self[0], self[1], self[2], self[3]))
    }
}

/// Conversion trait for values that can be interpreted as a margin pair.
pub trait IntoMarginPair {
    /// Converts the value into a pair of margin lengths.
    ///
    /// # Returns
    /// - `Some((Length, Length))` when the value can be expanded into a pair.
    /// - `None` when the value semantically represents the absence of margins.
    fn into_margin_pair(self) -> Option<(Length, Length)>;
}

/// Expands a single value into a symmetric margin pair.
impl<T> IntoMarginPair for T
where
    T: Into<Length> + Copy,
{
    /// Uses the same margin value for both elements of the pair.
    #[inline]
    fn into_margin_pair(self) -> Option<(Length, Length)> {
        Some((self.into(), self.into()))
    }
}

/// Converts a two-value tuple into a margin pair.
impl<T> IntoMarginPair for (T, T)
where
    T: Into<Length> + Copy,
{
    /// Interprets the tuple elements directly without reordering.
    #[inline]
    fn into_margin_pair(self) -> Option<(Length, Length)> {
        Some((self.0.into(), self.1.into()))
    }
}

/// Array-based shorthand for margin pair definitions.
impl<T> IntoMarginPair for [T; 2]
where
    T: Into<Length> + Copy,
{
    /// Delegates to the tuple-based margin pair conversion.
    #[inline]
    fn into_margin_pair(self) -> Option<(Length, Length)> {
        IntoMarginPair::into_margin_pair((self[0], self[1]))
    }
}
