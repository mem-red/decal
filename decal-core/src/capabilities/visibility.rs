use super::Drawable;

pub trait Visibility: Drawable {
    fn visible(mut self, value: bool) -> Self {
        self.visual_mut().visible = value;
        self
    }
}
