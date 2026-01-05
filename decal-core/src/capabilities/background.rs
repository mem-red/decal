use super::Drawable;
use crate::attributes::IntoPaint;

pub trait Background: Drawable {
    fn background<T>(mut self, value: T) -> Self
    where
        T: IntoPaint,
    {
        let background = value.into_paint().unwrap_or(crate::primitives::Paint::None);
        self.visual_mut().background = background.clone();
        self.add_resources(background);

        self
    }

    //

    fn bg<T>(self, value: T) -> Self
    where
        T: IntoPaint,
    {
        self.background(value)
    }
}
