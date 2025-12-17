use super::Drawable;

pub trait SelfAlignment: Drawable {
    fn align_self<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Option<crate::primitives::AlignSelf>>,
    {
        self.layout_mut().align_self = value.into().map(|x| x.into());
        self
    }

    fn justify_self<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Option<crate::primitives::JustifySelf>>,
    {
        self.layout_mut().justify_self = value.into().map(|x| x.into());
        self
    }

    fn flex_basis<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Option<crate::primitives::Length>>,
    {
        self.layout_mut().flex_basis = value
            .into()
            .map(|x| x.into())
            .unwrap_or(taffy::Dimension::auto());
        self
    }

    fn flex_grow<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Option<f32>>,
    {
        self.layout_mut().flex_grow = value.into().unwrap_or(0.0);
        self
    }

    fn flex_shrink<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Option<f32>>,
    {
        self.layout_mut().flex_shrink = value.into().unwrap_or(1.0);
        self
    }
}
