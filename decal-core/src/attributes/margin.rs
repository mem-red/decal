use crate::primitives::{
    Length,
    Rect,
};

type Margin = Rect<Length>;

pub trait IntoMargin {
    fn into_margin(self) -> Option<Margin>;
}

impl IntoMargin for Option<Margin> {
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        self
    }
}

impl IntoMargin for Margin {
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(self)
    }
}

impl<T> IntoMargin for T
where
    T: Into<Length> + Copy,
{
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(Rect {
            top: self.into(),
            right: self.into(),
            bottom: self.into(),
            left: self.into(),
        })
    }
}

impl<T> IntoMargin for [T; 1]
where
    T: Into<Length> + Copy,
{
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(Rect {
            top: self[0].into(),
            right: self[0].into(),
            bottom: self[0].into(),
            left: self[0].into(),
        })
    }
}

impl<T> IntoMargin for [T; 2]
where
    T: Into<Length> + Copy,
{
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(Rect {
            top: self[0].into(),
            right: self[1].into(),
            bottom: self[0].into(),
            left: self[1].into(),
        })
    }
}

impl<T> IntoMargin for [T; 3]
where
    T: Into<Length> + Copy,
{
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(Rect {
            top: self[0].into(),
            right: self[1].into(),
            bottom: self[2].into(),
            left: self[1].into(),
        })
    }
}

impl<T> IntoMargin for [T; 4]
where
    T: Into<Length> + Copy,
{
    #[inline]
    fn into_margin(self) -> Option<Margin> {
        Some(Rect {
            top: self[0].into(),
            right: self[1].into(),
            bottom: self[2].into(),
            left: self[3].into(),
        })
    }
}

//

pub trait IntoMarginPair {
    fn into_margin_pair(self) -> Option<(Length, Length)>;
}

impl<T> IntoMarginPair for T
where
    T: Into<Length> + Copy,
{
    #[inline]
    fn into_margin_pair(self) -> Option<(Length, Length)> {
        Some((self.into(), self.into()))
    }
}

impl<T> IntoMarginPair for [T; 1]
where
    T: Into<Length> + Copy,
{
    #[inline]
    fn into_margin_pair(self) -> Option<(Length, Length)> {
        Some((self[0].into(), self[0].into()))
    }
}

impl<T> IntoMarginPair for [T; 2]
where
    T: Into<Length> + Copy,
{
    #[inline]
    fn into_margin_pair(self) -> Option<(Length, Length)> {
        Some((self[0].into(), self[1].into()))
    }
}
