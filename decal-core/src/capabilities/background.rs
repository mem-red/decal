use super::Drawable;
use crate::attributes::IntoPaintStack;

pub trait Background: Drawable {
    fn background<T>(mut self, value: T) -> Self
    where
        T: IntoPaintStack,
    {
        let background = value.into_paint_stack();
        self.visual_mut().background = background.clone();
        self.add_resources(background);

        self
    }

    //

    fn bg<T>(self, value: T) -> Self
    where
        T: IntoPaintStack,
    {
        self.background(value)
    }
}
