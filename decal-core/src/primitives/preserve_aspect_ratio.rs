use crate::utils::IsDefault;
use enum_display::EnumDisplay;
use std::fmt::{
    Display,
    Formatter,
};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, EnumDisplay)]
enum Align {
    #[display("none")]
    None,
    #[display("xMinYMin")]
    XMinYMin,
    #[display("xMidYMin")]
    XMidYMin,
    #[display("xMaxYMin")]
    XMaxYMin,
    #[display("xMinYMid")]
    XMinYMid,
    #[display("xMidYMid")]
    XMidYMid,
    #[display("xMaxYMid")]
    XMaxYMid,
    #[display("xMinYMax")]
    XMinYMax,
    #[display("xMidYMax")]
    XMidYMax,
    #[display("xMaxYMax")]
    XMaxYMax,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, EnumDisplay)]
enum MeetOrSlice {
    #[display("meet")]
    Meet,
    #[display("slice")]
    Slice,
}

/// Controls how a view box is scaled and aligned within its viewport.
///
/// # Reference
///
/// https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/preserveAspectRatio
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct PreserveAspectRatio(Align, MeetOrSlice);

impl Default for PreserveAspectRatio {
    fn default() -> Self {
        Self(Align::XMidYMid, MeetOrSlice::Meet)
    }
}

impl IsDefault for PreserveAspectRatio {}

impl PreserveAspectRatio {
    /// Creates a new [`PreserveAspectRatio`] instance.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Disables aspect ratio preservation (non-uniform scaling).
    ///
    /// # Returns
    /// - [`Self`]
    pub fn align_none(mut self) -> Self {
        self.0 = Align::None;
        self
    }

    /// Aligns the view box to the top-left corner of the viewport.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn x_min_y_min(mut self) -> Self {
        self.0 = Align::XMinYMin;
        self
    }
    /// Aligns the view box to the top-center of the viewport.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn x_mid_y_min(mut self) -> Self {
        self.0 = Align::XMidYMin;
        self
    }

    /// Aligns the view box to the top-right corner of the viewport.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn x_max_y_min(mut self) -> Self {
        self.0 = Align::XMaxYMin;
        self
    }

    /// Aligns the view box to the middle-left of the viewport.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn x_min_y_mid(mut self) -> Self {
        self.0 = Align::XMinYMid;
        self
    }

    /// Aligns the view box to the center of the viewport.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn x_mid_y_mid(mut self) -> Self {
        self.0 = Align::XMidYMid;
        self
    }

    /// Aligns the view box to the middle-right of the viewport.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn x_max_y_mid(mut self) -> Self {
        self.0 = Align::XMaxYMid;
        self
    }

    /// Aligns the view box to the bottom-left corner of the viewport.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn x_min_y_max(mut self) -> Self {
        self.0 = Align::XMinYMax;
        self
    }

    /// Aligns the view box to the bottom-center of the viewport.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn x_mid_y_max(mut self) -> Self {
        self.0 = Align::XMidYMax;
        self
    }

    /// Aligns the view box to the bottom-right corner of the viewport.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn x_max_y_max(mut self) -> Self {
        self.0 = Align::XMaxYMax;
        self
    }

    /// Scales the view box to fit entirely within the viewport. This preserves
    /// the entire content without clipping.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn meet(mut self) -> Self {
        self.1 = MeetOrSlice::Meet;
        self
    }

    /// Scales the view box to fill the viewport completely. This may clip parts
    /// of the content.
    ///
    /// # Returns
    /// - [`Self`]
    pub fn slice(mut self) -> Self {
        self.1 = MeetOrSlice::Slice;
        self
    }
}

impl Display for PreserveAspectRatio {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}
