use crate::primitives::{
    Corner,
    Length,
};

/// Type alias representing the radius of a single corner.
type CornerRadii = Length<false, true>;

/// Type alias for a structure holding four independent corner radii.
pub type CornerRadius = Corner<CornerRadii>;

/// Conversion trait for values that can be interpreted as a full corner radius
/// definition.
pub trait IntoCornerRadius {
    /// Converts the value into a corner radius structure.
    ///
    /// This abstraction allows a wide range of ergonomic inputs while keeping
    /// call sites concise.
    ///
    /// # Returns
    /// - `Some(CornerRadius)` when the value can be expanded into four corner
    ///   radii.
    /// - `None` when the value semantically represents the absence of corner
    ///   rounding.
    fn into_corner_radius(self) -> Option<CornerRadius>;
}

/// Identity conversion for an optional corner radius.
impl IntoCornerRadius for Option<CornerRadius> {
    /// Returns the contained corner radius without modification.
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        self
    }
}

/// Wraps an existing corner radius into an optional value.
impl IntoCornerRadius for CornerRadius {
    /// Converts a concrete corner radius into `Some(CornerRadius)`.
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(self)
    }
}

/// Expands a single value into a uniform radius applied to all corners.
impl<T> IntoCornerRadius for T
where
    T: Into<CornerRadii> + Copy,
{
    /// Uses the same radius for all four corners.
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(Corner {
            top_left: self.into(),
            top_right: self.into(),
            bottom_right: self.into(),
            bottom_left: self.into(),
        })
    }
}

/// Converts a two-value tuple into alternating corner radii.
impl<T> IntoCornerRadius for (T, T)
where
    T: Into<CornerRadii> + Copy,
{
    /// Interprets the tuple as `(first, second)` applied diagonally.
    ///
    /// The first value is used for top-left and bottom-right, the second for
    /// top-right and bottom-left.
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(Corner {
            top_left: self.0.into(),
            top_right: self.1.into(),
            bottom_right: self.0.into(),
            bottom_left: self.1.into(),
        })
    }
}

/// Converts a three-value tuple into a corner radius with an inferred
/// bottom-left value.
impl<T> IntoCornerRadius for (T, T, T)
where
    T: Into<CornerRadii> + Copy,
{
    /// Interprets the tuple as `(top_left, top_right, bottom_right)`.
    ///
    /// The bottom-left corner reuses the top-right value.
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(Corner {
            top_left: self.0.into(),
            top_right: self.1.into(),
            bottom_right: self.2.into(),
            bottom_left: self.1.into(),
        })
    }
}

/// Converts a four-value tuple into an explicit radius for each corner.
impl<T> IntoCornerRadius for (T, T, T, T)
where
    T: Into<CornerRadii> + Copy,
{
    /// Interprets the tuple as `(top_left, top_right, bottom_right,
    /// bottom_left)` in clockwise order.
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        Some(Corner {
            top_left: self.0.into(),
            top_right: self.1.into(),
            bottom_right: self.2.into(),
            bottom_left: self.3.into(),
        })
    }
}

/// Array-based shorthand for two-value corner radius definitions.
impl<T> IntoCornerRadius for [T; 2]
where
    T: Into<CornerRadii> + Copy,
{
    /// Delegates to the two-element tuple implementation.
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        IntoCornerRadius::into_corner_radius((self[0], self[1]))
    }
}

/// Array-based shorthand for three-value corner radius definitions.
impl<T> IntoCornerRadius for [T; 3]
where
    T: Into<CornerRadii> + Copy,
{
    /// Delegates to the three-element tuple implementation.
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        IntoCornerRadius::into_corner_radius((self[0], self[1], self[2]))
    }
}

/// Array-based shorthand for four-value corner radius definitions.
impl<T> IntoCornerRadius for [T; 4]
where
    T: Into<CornerRadii> + Copy,
{
    /// Delegates to the four-element tuple implementation.
    #[inline]
    fn into_corner_radius(self) -> Option<CornerRadius> {
        IntoCornerRadius::into_corner_radius((self[0], self[1], self[2], self[3]))
    }
}
