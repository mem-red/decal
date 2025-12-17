use super::Drawable;

pub trait Visibility: Drawable {
    fn visible(&mut self, value: bool) -> &mut Self {
        self.visual_mut().visible = value;
        self
    }
}
