/// Controls whether flex wrap behavior.
#[derive(Debug, Copy, Clone)]
pub enum FlexWrap {
    /// Items will not wrap, even if they overflow the container.
    NoWrap,
    /// Allows items to wrap.
    Wrap,
    /// Allows items to wrap in the reverse direction.
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
