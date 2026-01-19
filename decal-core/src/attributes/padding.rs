use crate::primitives::{
    Length,
    Rect,
};

type PaddingValue = Length<false, true>;
type Padding = Rect<PaddingValue>;

pub trait IntoPadding {
    fn into_padding(self) -> Option<Padding>;
}

impl IntoPadding for Option<Padding> {
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        self
    }
}

impl IntoPadding for Padding {
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(self)
    }
}

impl<T> IntoPadding for T
where
    T: Into<PaddingValue> + Copy,
{
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(Rect {
            top: self.into(),
            right: self.into(),
            bottom: self.into(),
            left: self.into(),
        })
    }
}

impl<T> IntoPadding for [T; 1]
where
    T: Into<PaddingValue> + Copy,
{
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(Rect {
            top: self[0].into(),
            right: self[0].into(),
            bottom: self[0].into(),
            left: self[0].into(),
        })
    }
}

impl<T> IntoPadding for [T; 2]
where
    T: Into<PaddingValue> + Copy,
{
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(Rect {
            top: self[0].into(),
            right: self[1].into(),
            bottom: self[0].into(),
            left: self[1].into(),
        })
    }
}

impl<T> IntoPadding for [T; 3]
where
    T: Into<PaddingValue> + Copy,
{
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(Rect {
            top: self[0].into(),
            right: self[1].into(),
            bottom: self[2].into(),
            left: self[1].into(),
        })
    }
}

impl<T> IntoPadding for [T; 4]
where
    T: Into<PaddingValue> + Copy,
{
    #[inline]
    fn into_padding(self) -> Option<Padding> {
        Some(Rect {
            top: self[0].into(),
            right: self[1].into(),
            bottom: self[2].into(),
            left: self[3].into(),
        })
    }
}

//

type PaddingPair = (PaddingValue, PaddingValue);

pub trait IntoPaddingPair {
    fn into_padding_pair(self) -> Option<PaddingPair>;
}

impl<T> IntoPaddingPair for T
where
    T: Into<PaddingValue> + Copy,
{
    #[inline]
    fn into_padding_pair(self) -> Option<PaddingPair> {
        Some((self.into(), self.into()))
    }
}

impl<T> IntoPaddingPair for [T; 1]
where
    T: Into<PaddingValue> + Copy,
{
    #[inline]
    fn into_padding_pair(self) -> Option<PaddingPair> {
        Some((self[0].into(), self[0].into()))
    }
}

impl<T> IntoPaddingPair for [T; 2]
where
    T: Into<PaddingValue> + Copy,
{
    #[inline]
    fn into_padding_pair(self) -> Option<PaddingPair> {
        Some((self[0].into(), self[1].into()))
    }
}
