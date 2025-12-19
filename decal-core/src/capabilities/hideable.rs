use super::Drawable;

pub trait Hideable: Drawable {
    fn hidden(self, value: bool) -> Self;

    //

    fn hide(self) -> Self {
        self.hidden(true)
    }

    fn show(self) -> Self {
        self.hidden(false)
    }
}
