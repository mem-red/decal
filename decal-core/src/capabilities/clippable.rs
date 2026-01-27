use super::Drawable;
use crate::{
    attributes::IntoOverflow,
    primitives::Overflow,
};

macro_rules! impl_axis {
    ($method:ident, $field:ident) => {
        #[doc = concat!("Sets the overflow behavior for the `", stringify!($field), "` axis.")]
        #[doc = ""]
        #[doc = "# Arguments"]
        #[doc = "- `value`: The [`Overflow`] behavior."]
        #[doc = ""]
        #[doc = "# Returns"]
        #[doc = "- [`Self`]"]
        fn $method<T>(mut self, value: T) -> Self
        where
            T: Into<Option<Overflow>>,
        {
            self.layout_mut().overflow.$field = value.into().unwrap_or_default().into();
            self
        }
    };
}

/// Capability for configuring overflow and clipping behavior on a node.
pub trait Clippable: Drawable {
    /// Sets the overflow behavior for both axes.
    ///
    /// # Arguments
    /// - `value`: The overflow configuration convertible using
    ///   [`IntoOverflow`].
    ///
    /// # Returns
    /// - [`Self`]
    fn overflow<T>(mut self, value: T) -> Self
    where
        T: IntoOverflow,
    {
        self.layout_mut().overflow = value.into_overflow().unwrap_or_default().into();
        self
    }

    impl_axis!(overflow_x, x);
    impl_axis!(overflow_y, y);

    /// Sets overflow to [`Overflow::Hidden`] on both axes.
    ///
    /// # Returns
    /// - [`Self`]
    fn overflow_hidden(self) -> Self {
        self.overflow(Overflow::Hidden)
    }

    /// Sets overflow to [`Overflow::Visible`] on both axes.
    ///
    /// # Returns
    /// - [`Self`]
    fn overflow_visible(self) -> Self {
        self.overflow(Overflow::Visible)
    }
}
