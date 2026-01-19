use crate::utils::IsDefault;
use enum_display::EnumDisplay;
use std::fmt::{
    Display,
    Formatter,
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, EnumDisplay)]
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

#[derive(Debug, Hash, Eq, PartialEq, Clone, EnumDisplay)]
enum MeetOrSlice {
    #[display("meet")]
    Meet,
    #[display("slice")]
    Slice,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct PreserveAspectRatio(Align, MeetOrSlice);

impl Default for PreserveAspectRatio {
    fn default() -> Self {
        Self(Align::XMidYMid, MeetOrSlice::Meet)
    }
}

impl IsDefault for PreserveAspectRatio {}

impl PreserveAspectRatio {
    pub fn new() -> Self {
        Self::default()
    }

    //

    pub fn align_none(mut self) -> Self {
        self.0 = Align::None;
        self
    }

    pub fn x_min_y_min(mut self) -> Self {
        self.0 = Align::XMinYMin;
        self
    }

    pub fn x_mid_y_min(mut self) -> Self {
        self.0 = Align::XMidYMin;
        self
    }

    pub fn x_max_y_min(mut self) -> Self {
        self.0 = Align::XMaxYMin;
        self
    }

    pub fn x_min_y_mid(mut self) -> Self {
        self.0 = Align::XMinYMid;
        self
    }

    pub fn x_mid_y_mid(mut self) -> Self {
        self.0 = Align::XMidYMid;
        self
    }

    pub fn x_max_y_mid(mut self) -> Self {
        self.0 = Align::XMaxYMid;
        self
    }

    pub fn x_min_y_max(mut self) -> Self {
        self.0 = Align::XMinYMax;
        self
    }

    pub fn x_mid_y_max(mut self) -> Self {
        self.0 = Align::XMidYMax;
        self
    }

    pub fn x_max_y_max(mut self) -> Self {
        self.0 = Align::XMaxYMax;
        self
    }

    //

    pub fn meet(mut self) -> Self {
        self.1 = MeetOrSlice::Meet;
        self
    }

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
