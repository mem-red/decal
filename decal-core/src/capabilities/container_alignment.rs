use super::Drawable;

pub trait ContainerAlignment: Drawable {
    fn align_items<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Option<crate::primitives::AlignItems>>,
    {
        self.layout_mut().align_items = value.into().map(|x| x.into());
        self
    }

    fn align_content<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Option<crate::primitives::AlignContent>>,
    {
        self.layout_mut().align_content = value.into().map(|x| x.into());
        self
    }

    fn justify_content<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Option<crate::primitives::JustifyContent>>,
    {
        self.layout_mut().justify_content = value.into().map(|x| x.into());
        self
    }
}
