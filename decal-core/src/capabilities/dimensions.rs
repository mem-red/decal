use super::Drawable;
use crate::{
    attributes::IntoDimensions,
    primitives::IntoOptionalLength,
};

macro_rules! impl_dimension {
    ($method:ident, $taffy_method:ident, $taffy_field:ident) => {
        #[doc = concat!("Sets the ", stringify!($field), " dimension of the node.")]
        #[doc = ""]
        #[doc = "# Arguments"]
        #[doc = "- `value`: The dimension value convertible using [`IntoOptionalLength`]."]
        #[doc = ""]
        #[doc = "# Returns"]
        #[doc = "- [`Self`]"]
        fn $method<T>(mut self, value: T) -> Self
        where
            T: IntoOptionalLength,
        {
            self.layout_mut().$taffy_method.$taffy_field = value
                .into_optional_length()
                .map_or(taffy::Dimension::auto(), |inner| inner.into());
            self
        }
    };
}

/// Capability for configuring size constraints on a node.
pub trait Dimensions: Drawable {
    /// Sets the explicit width and height of the node.
    ///
    /// # Arguments
    /// - `value`: The size definition convertible using [`IntoDimensions`].
    ///
    /// # Returns
    /// - [`Self`]
    fn size<T>(mut self, value: T) -> Self
    where
        T: IntoDimensions,
    {
        self.layout_mut().size = value
            .into_dimensions()
            .map_or(taffy::Size::auto(), |inner| inner.into());
        self
    }

    /// Sets the minimum width and height constraints of the node.
    ///
    /// # Arguments
    /// - `value`: The minimum size definition convertible using
    ///   [`IntoDimensions`].
    ///
    /// # Returns
    /// - [`Self`]
    fn min_size<T>(mut self, value: T) -> Self
    where
        T: IntoDimensions,
    {
        self.layout_mut().min_size = value
            .into_dimensions()
            .map_or(taffy::Size::auto(), |inner| inner.into());
        self
    }

    /// Sets the maximum width and height constraints of the node.
    ///
    /// # Arguments
    /// - `value`: The maximum size definition convertible using
    ///   [`IntoDimensions`].
    ///
    /// # Returns
    /// - [`Self`]
    fn max_size<T>(mut self, value: T) -> Self
    where
        T: IntoDimensions,
    {
        self.layout_mut().max_size = value
            .into_dimensions()
            .map_or(taffy::Size::auto(), |inner| inner.into());
        self
    }

    impl_dimension!(width, size, width);
    impl_dimension!(height, size, height);

    impl_dimension!(min_width, min_size, width);
    impl_dimension!(min_height, min_size, height);

    impl_dimension!(max_width, max_size, width);
    impl_dimension!(max_height, max_size, height);
}
