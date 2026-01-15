use super::Drawable;
use crate::filters::Filter;

pub trait FilterEffects: Drawable {
    fn fx<T>(mut self, value: T) -> Self
    where
        T: Into<Filter>,
    {
        let filter = value.into();
        self.visual_mut().filter = filter.clone();
        self.add_resources(filter);
        self
    }
}
