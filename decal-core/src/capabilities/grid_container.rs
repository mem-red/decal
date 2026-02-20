use super::Drawable;
use crate::primitives::JustifyItems;
use taffy::style::{
    GridAutoFlow,
    GridTemplateArea,
    GridTemplateComponent,
    TrackSizingFunction,
};

/// Capability for configuring grid container behavior on a node.
pub trait GridContainer: Drawable {
    /// Sets the track sizing functions (heights) of the grid rows.
    ///
    /// # Arguments
    /// - `value`: The list describing each row's sizing function.
    ///
    /// # Returns
    /// - [`Self`]
    fn grid_template_rows(mut self, value: Vec<GridTemplateComponent<String>>) -> Self {
        self.layout_mut().grid_template_rows = value;
        self
    }

    /// Sets the track sizing functions (widths) of the grid columns.
    ///
    /// # Arguments
    /// - `value`: The list describing each column's sizing function.
    ///
    /// # Returns
    /// - [`Self`]
    fn grid_template_columns(mut self, value: Vec<GridTemplateComponent<String>>) -> Self {
        self.layout_mut().grid_template_columns = value;
        self
    }

    /// Sets the size of implicitly created grid rows.
    ///
    /// # Arguments
    /// - `value`: The list used to size rows that are generated automatically.
    ///
    /// # Returns
    /// - [`Self`]
    fn grid_auto_rows(mut self, value: Vec<TrackSizingFunction>) -> Self {
        self.layout_mut().grid_auto_rows = value;
        self
    }

    /// Sets the size of implicitly created grid columns.
    ///
    /// # Arguments
    /// - `value`: The list used to size columns that are generated
    ///   automatically.
    ///
    /// # Returns
    /// - [`Self`]
    fn grid_auto_columns(mut self, value: Vec<TrackSizingFunction>) -> Self {
        self.layout_mut().grid_auto_columns = value;
        self
    }

    /// Sets how auto-placed items are inserted into the grid.
    ///
    /// # Arguments
    /// - `value`: The [`GridAutoFlow`] behavior.
    ///
    /// # Returns
    /// - [`Self`]
    fn grid_auto_flow(mut self, value: GridAutoFlow) -> Self {
        self.layout_mut().grid_auto_flow = value;
        self
    }

    /// Sets the rectangular grid template areas.
    ///
    /// # Arguments
    /// - `value`: A list describing the named areas.
    ///
    /// # Returns
    /// - [`Self`]
    fn grid_template_areas(mut self, value: Vec<GridTemplateArea<String>>) -> Self {
        self.layout_mut().grid_template_areas = value;
        self
    }

    /// Sets the named grid lines between columns.
    ///
    /// # Arguments
    /// - `value`: A list of line names.
    ///
    /// # Returns
    /// - [`Self`]
    fn grid_template_column_names(mut self, value: Vec<Vec<String>>) -> Self {
        self.layout_mut().grid_template_column_names = value;
        self
    }

    /// Sets the named grid lines between rows.
    ///
    /// # Arguments
    /// - `value`: A list of line names.
    ///
    /// # Returns
    /// - [`Self`]
    fn grid_template_row_names(mut self, value: Vec<Vec<String>>) -> Self {
        self.layout_mut().grid_template_row_names = value;
        self
    }

    /// Sets how grid items are aligned along the inline axis within their grid
    /// areas.
    ///
    /// # Arguments
    /// - `value`: The [`JustifyItems`] behavior.
    ///
    /// # Returns
    /// - [`Self`]
    fn justify_items<T>(mut self, value: T) -> Self
    where
        T: Into<Option<JustifyItems>>,
    {
        self.layout_mut().justify_items = value.into().map(Into::into);
        self
    }
}
