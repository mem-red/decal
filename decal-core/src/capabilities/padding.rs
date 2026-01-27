use super::Drawable;
use crate::{
    attributes::{
        IntoPadding,
        IntoPaddingPair,
    },
    primitives::IntoOptionalLength,
};

macro_rules! impl_side {
    ($method:ident, $field:ident) => {
        #[doc = concat!("Sets the padding for the `", stringify!($field), "` side.")]
        #[doc = ""]
        #[doc = "# Arguments"]
        #[doc = "- `value`: The padding value convertible using [`IntoOptionalLength`]."]
        #[doc = ""]
        #[doc = "# Returns"]
        #[doc = "- [`Self`]"]
        fn $method<T>(mut self, value: T) -> Self
        where
            T: IntoOptionalLength<false, true>,
        {
            self.layout_mut().padding.$field =
                value.into_optional_length().unwrap_or_default().into();
            self
        }
    };
}

/// Capability for configuring padding inside a node.
pub trait Padding: Drawable {
    /// Sets the padding for all four sides of the node.
    ///
    /// # Arguments
    /// - `value`: The padding definition convertible using [`IntoPadding`].
    ///
    /// # Returns
    /// - [`Self`]
    fn padding<T>(mut self, value: T) -> Self
    where
        T: IntoPadding,
    {
        self.layout_mut().padding = value.into_padding().unwrap_or_default().into();
        self
    }

    /// Sets the horizontal padding for the left and right sides.
    ///
    /// # Arguments
    /// - `value`: The horizontal padding values convertible using
    ///   [`IntoPaddingPair`].
    ///
    /// # Returns
    /// - [`Self`]
    fn padding_x<T>(mut self, value: T) -> Self
    where
        T: IntoPaddingPair,
    {
        let (left, right) = value.into_padding_pair().unwrap_or_default();
        self.layout_mut().padding.left = left.into();
        self.layout_mut().padding.right = right.into();
        self
    }

    /// Sets the vertical padding for the top and bottom sides.
    ///
    /// # Arguments
    /// - `value`: The vertical padding values convertible using
    ///   [`IntoPaddingPair`].
    ///
    /// # Returns
    /// - [`Self`]
    fn padding_y<T>(mut self, value: T) -> Self
    where
        T: IntoPaddingPair,
    {
        let (top, bottom) = value.into_padding_pair().unwrap_or_default();
        self.layout_mut().padding.top = top.into();
        self.layout_mut().padding.bottom = bottom.into();
        self
    }

    impl_side!(padding_top, top);
    impl_side!(padding_right, right);
    impl_side!(padding_bottom, bottom);
    impl_side!(padding_left, left);
}
