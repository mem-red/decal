#[derive(Debug, Copy, Clone)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Into<taffy::FlexWrap> for FlexWrap {
    #[inline]
    fn into(self) -> taffy::FlexWrap {
        match self {
            FlexWrap::NoWrap => taffy::FlexWrap::NoWrap,
            FlexWrap::Wrap => taffy::FlexWrap::Wrap,
            FlexWrap::WrapReverse => taffy::FlexWrap::WrapReverse,
        }
    }
}
