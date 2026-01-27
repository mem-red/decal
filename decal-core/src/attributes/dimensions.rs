use crate::primitives::{
    Length,
    Size,
};

/// Type alias representing two-dimensional size values expressed using lengths.
type Dimensions = Size<Length>;

/// Conversion trait for values that can be interpreted as width and height
/// dimensions.
pub trait IntoDimensions {
    /// Converts the value into a size definition.
    ///
    /// This abstraction allows a wide range of ergonomic inputs while keeping
    /// call sites concise.
    ///
    /// # Returns
    /// - `Some(Dimensions)` when the value can be expanded into width and
    ///   height.
    /// - `None` when the value semantically represents the absence of
    ///   dimensions.
    fn into_dimensions(self) -> Option<Dimensions>;
}

/// Identity conversion for an optional dimensions value.
impl IntoDimensions for Option<Dimensions> {
    /// Returns the contained dimensions without modification.
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        self
    }
}

/// Wraps an existing dimensions value into an optional.
impl IntoDimensions for Dimensions {
    /// Converts a concrete dimensions value into `Some(Dimensions)`.
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        Some(self)
    }
}

/// Expands a single value into equal width and height dimensions.
impl<T> IntoDimensions for T
where
    T: Into<Length> + Copy,
{
    /// Uses the same length for both width and height.
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        Some(Size {
            width: self.into(),
            height: self.into(),
        })
    }
}

/// Converts a two-value tuple into explicit width and height dimensions.
impl<T> IntoDimensions for (T, T)
where
    T: Into<Length> + Copy,
{
    /// Interprets the tuple as `(width, height)` without reordering.
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        Some(Size {
            width: self.0.into(),
            height: self.1.into(),
        })
    }
}

/// Array-based shorthand for width and height dimensions.
impl<T> IntoDimensions for [T; 2]
where
    T: Into<Length> + Copy,
{
    /// Delegates to the two-element tuple implementation.
    #[inline]
    fn into_dimensions(self) -> Option<Dimensions> {
        IntoDimensions::into_dimensions((self[0], self[1]))
    }
}
