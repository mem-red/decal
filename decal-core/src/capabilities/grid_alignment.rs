use super::Drawable;

pub trait GridAlignment: Drawable {
    fn justify_items<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Option<crate::primitives::JustifyItems>>,
    {
        self.layout_mut().justify_items = value.into().map(|x| x.into());
        self
    }
}
