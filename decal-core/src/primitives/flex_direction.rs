/// The direction of the main axis in a flex layout.
#[derive(Debug, Copy, Clone, Default)]
pub enum FlexDirection {
    /// Lays out items horizontally from left to right.
    #[default]
    Row,
    /// Lays out items vertically from top to bottom.
    Column,
    /// Lays out items horizontally from right to left.
    RowReverse,
    /// Lays out items vertically from bottom to top.
    ColumnReverse,
}

impl Into<taffy::FlexDirection> for FlexDirection {
    fn into(self) -> taffy::FlexDirection {
        match self {
            FlexDirection::Row => taffy::FlexDirection::Row,
            FlexDirection::Column => taffy::FlexDirection::Column,
            FlexDirection::RowReverse => taffy::FlexDirection::RowReverse,
            FlexDirection::ColumnReverse => taffy::FlexDirection::ColumnReverse,
        }
    }
}
