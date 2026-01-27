use super::Drawable;
use crate::{
    attributes::{
        IntoBorder,
        IntoBorderPair,
        IntoPaintStack,
    },
    primitives::IntoOptionalLength,
};

macro_rules! impl_side {
    ($method:ident, $field:ident) => {
        #[doc = concat!("Sets the border width for the `", stringify!($field), "` side.")]
        #[doc = ""]
        #[doc = "# Arguments"]
        #[doc = "- `value`: The border width convertible using [`IntoOptionalLength`]."]
        #[doc = ""]
        #[doc = "# Returns"]
        #[doc = "- [`Self`]"]
        fn $method<T>(mut self, value: T) -> Self
        where
            T: IntoOptionalLength<false, true>,
        {
            self.layout_mut().border.$field =
                value.into_optional_length().unwrap_or_default().into();
            self
        }
    };
}

/// Capability for configuring border width and border paint on a node.
pub trait Border: Drawable {
    /// Sets the border width for all four sides of the node.
    ///
    /// # Arguments
    /// - `value`: The border width definition convertible using [`IntoBorder`].
    ///
    /// # Returns
    /// - [`Self`]
    fn border_width<T>(mut self, value: T) -> Self
    where
        T: IntoBorder,
    {
        self.layout_mut().border = value.into_border().unwrap_or_default().into();
        self
    }

    /// Sets the horizontal border widths for the left and right sides.
    ///
    /// # Arguments
    /// - `value`: The horizontal border widths convertible using
    ///   [`IntoBorderPair`].
    ///
    /// # Returns
    /// - [`Self`]
    fn border_x_width<T>(mut self, value: T) -> Self
    where
        T: IntoBorderPair,
    {
        let (left, right) = value.into_border_pair().unwrap_or_default();
        self.layout_mut().border.left = left.into();
        self.layout_mut().border.right = right.into();
        self
    }

    /// Sets the vertical border widths for the top and bottom sides.
    ///
    /// # Arguments
    /// - `value`: The vertical border widths convertible using
    ///   [`IntoBorderPair`].
    ///
    /// # Returns
    /// - [`Self`]
    fn border_y_width<T>(mut self, value: T) -> Self
    where
        T: IntoBorderPair,
    {
        let (top, bottom) = value.into_border_pair().unwrap_or_default();
        self.layout_mut().border.top = top.into();
        self.layout_mut().border.bottom = bottom.into();
        self
    }

    impl_side!(border_top_width, top);
    impl_side!(border_right_width, right);
    impl_side!(border_bottom_width, bottom);
    impl_side!(border_left_width, left);

    /// Sets the border paint.
    ///
    /// # Arguments
    /// - `value`: The border paint configuration convertible into a paint stack
    ///   using [`IntoPaintStack`].
    ///
    /// # Returns
    /// - [`Self`]
    fn border<T>(mut self, value: T) -> Self
    where
        T: IntoPaintStack,
    {
        let border = value.into_paint_stack();
        self.visual_mut().border = border.clone();
        self.add_resources(border);
        self
    }
}
