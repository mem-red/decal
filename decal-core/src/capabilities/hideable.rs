use super::Drawable;

pub trait Hideable: Drawable {
    fn hidden(&mut self, value: bool) -> &mut Self;

    //

    fn hide(&mut self) -> &mut Self {
        self.hidden(true);
        self
    }

    fn show(&mut self) -> &mut Self {
        self.hidden(false);
        self
    }
}
