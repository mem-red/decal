use super::Drawable;
use crate::primitives::FlexWrap;

pub trait FlexContainer: Drawable {
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

    fn flex_wrap(mut self, value: FlexWrap) -> Self {
        self.layout_mut().flex_wrap = value.into();
        self
    }

    //

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
