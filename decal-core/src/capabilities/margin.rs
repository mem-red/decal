use super::Drawable;
use crate::{
    attributes::{
        IntoMargin,
        IntoMarginPair,
    },
    primitives::IntoOptionalLength,
};

macro_rules! impl_side {
    ($method:ident, $field:ident) => {
        #[doc = concat!("Sets the margin for the `", stringify!($field), "` side.")]
        #[doc = ""]
        #[doc = "# Arguments"]
        #[doc = "- `value`: The margin value convertible using [`IntoOptionalLength`]."]
        #[doc = ""]
        #[doc = "# Returns"]
        #[doc = "- [`Self`]"]
        fn $method<T>(mut self, value: T) -> Self
        where
            T: IntoOptionalLength,
        {
            self.layout_mut().margin.$field =
                value.into_optional_length().unwrap_or_default().into();
            self
        }
    };
}

/// Capability for configuring margin around a node.
pub trait Margin: Drawable {
    /// Sets the margin for all four sides of the node.
    ///
    /// # Arguments
    /// - `value`: The margin definition convertible using [`IntoMargin`].
    ///
    /// # Returns
    /// - [`Self`]
    fn margin<T>(mut self, value: T) -> Self
    where
        T: IntoMargin,
    {
        self.layout_mut().margin = value.into_margin().unwrap_or_default().into();
        self
    }

    /// Sets the horizontal margins for the left and right sides.
    ///
    /// # Arguments
    /// - `value`: The horizontal margin values convertible using
    ///   [`IntoMarginPair`].
    ///
    /// # Returns
    /// - [`Self`]
    fn margin_x<T>(mut self, value: T) -> Self
    where
        T: IntoMarginPair,
    {
        let (left, right) = value.into_margin_pair().unwrap_or_default();
        self.layout_mut().margin.left = left.into();
        self.layout_mut().margin.right = right.into();
        self
    }

    /// Sets the vertical margins for the top and bottom sides.
    ///
    /// # Arguments
    /// - `value`: The vertical margin values convertible using
    ///   [`IntoMarginPair`].
    ///
    /// # Returns
    /// - [`Self`]
    fn margin_y<T>(mut self, value: T) -> Self
    where
        T: IntoMarginPair,
    {
        let (top, bottom) = value.into_margin_pair().unwrap_or_default();
        self.layout_mut().margin.top = top.into();
        self.layout_mut().margin.bottom = bottom.into();
        self
    }

    impl_side!(margin_top, top);
    impl_side!(margin_right, right);
    impl_side!(margin_bottom, bottom);
    impl_side!(margin_left, left);
}
