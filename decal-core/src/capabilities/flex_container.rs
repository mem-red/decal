use super::Drawable;
use crate::primitives::FlexWrap;

/// Capability for configuring flex container behavior on a node.
pub trait FlexContainer: Drawable {
    /// Reverses or restores the current flex direction based on the provided
    /// flag.
    ///
    /// # Arguments
    /// - `reverse`: Whether the flex direction should be reversed.
    ///
    /// # Returns
    /// - [`Self`]
    fn reversed(mut self, reverse: bool) -> Self {
        self.layout_mut().flex_direction = if reverse {
            match self.layout().flex_direction {
                taffy::FlexDirection::Row => taffy::FlexDirection::RowReverse,
                taffy::FlexDirection::Column => taffy::FlexDirection::ColumnReverse,
                other => other,
            }
        } else {
            match self.layout().flex_direction {
                taffy::FlexDirection::RowReverse => taffy::FlexDirection::Row,
                taffy::FlexDirection::ColumnReverse => taffy::FlexDirection::Column,
                other => other,
            }
        };

        self
    }

    /// Sets how flex items wrap within the container.
    ///
    /// # Arguments
    /// - `value`: The [`FlexWrap`] behavior applied to the container.
    ///
    /// # Returns
    /// - [`Self`]
    fn flex_wrap(mut self, value: FlexWrap) -> Self {
        self.layout_mut().flex_wrap = value.into();
        self
    }

    /// Toggles the current flex direction between normal and reversed.
    ///
    /// This is a convenience method that flips the direction without requiring
    /// an explicit flag.
    ///
    /// # Returns
    /// - [`Self`]
    fn reverse(mut self) -> Self {
        self.layout_mut().flex_direction = match self.layout().flex_direction {
            taffy::FlexDirection::Row => taffy::FlexDirection::RowReverse,
            taffy::FlexDirection::RowReverse => taffy::FlexDirection::Row,
            taffy::FlexDirection::Column => taffy::FlexDirection::ColumnReverse,
            taffy::FlexDirection::ColumnReverse => taffy::FlexDirection::Column,
        };

        self
    }
}
