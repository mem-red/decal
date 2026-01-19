use crate::primitives::{
    Length,
    Rect,
};

type BorderWidth = Length<false, true>;
type Border = Rect<BorderWidth>;

pub trait IntoBorder {
    fn into_border(self) -> Option<Border>;
}

impl IntoBorder for Option<Border> {
    #[inline]
    fn into_border(self) -> Option<Border> {
        self
    }
}

impl IntoBorder for Border {
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(self)
    }
}

impl<T> IntoBorder for T
where
    T: Into<BorderWidth> + Copy,
{
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(Rect {
            top: self.into(),
            right: self.into(),
            bottom: self.into(),
            left: self.into(),
        })
    }
}

impl<T> IntoBorder for [T; 1]
where
    T: Into<BorderWidth> + Copy,
{
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(Rect {
            top: self[0].into(),
            right: self[0].into(),
            bottom: self[0].into(),
            left: self[0].into(),
        })
    }
}

impl<T> IntoBorder for [T; 2]
where
    T: Into<BorderWidth> + Copy,
{
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(Rect {
            top: self[0].into(),
            right: self[1].into(),
            bottom: self[0].into(),
            left: self[1].into(),
        })
    }
}

impl<T> IntoBorder for [T; 3]
where
    T: Into<BorderWidth> + Copy,
{
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(Rect {
            top: self[0].into(),
            right: self[1].into(),
            bottom: self[2].into(),
            left: self[1].into(),
        })
    }
}

impl<T> IntoBorder for [T; 4]
where
    T: Into<BorderWidth> + Copy,
{
    #[inline]
    fn into_border(self) -> Option<Border> {
        Some(Rect {
            top: self[0].into(),
            right: self[1].into(),
            bottom: self[2].into(),
            left: self[3].into(),
        })
    }
}

//

type BorderPair = (BorderWidth, BorderWidth);

pub trait IntoBorderPair {
    fn into_border_pair(self) -> Option<BorderPair>;
}

impl<T> IntoBorderPair for T
where
    T: Into<BorderWidth> + Copy,
{
    #[inline]
    fn into_border_pair(self) -> Option<BorderPair> {
        Some((self.into(), self.into()))
    }
}

impl<T> IntoBorderPair for [T; 1]
where
    T: Into<BorderWidth> + Copy,
{
    #[inline]
    fn into_border_pair(self) -> Option<BorderPair> {
        Some((self[0].into(), self[0].into()))
    }
}

impl<T> IntoBorderPair for [T; 2]
where
    T: Into<BorderWidth> + Copy,
{
    #[inline]
    fn into_border_pair(self) -> Option<BorderPair> {
        Some((self[0].into(), self[1].into()))
    }
}
