use super::Drawable;

pub trait AspectRatio: Drawable {
    const SQUARE: f32 = 1.0;
    const THREE_TWO: f32 = 3.0 / 2.0;
    const FOUR_THREE: f32 = 4.0 / 3.0;
    const SIXTEEN_NINE: f32 = 16.0 / 9.0;
    const NINE_SIXTEEN: f32 = 9.0 / 16.0;

    fn aspect_ratio<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Option<f32>>,
    {
        self.layout_mut().aspect_ratio = value.into();
        self
    }

    //

    fn aspect_square(&mut self) -> &mut Self {
        self.aspect_ratio(Self::SQUARE);
        self
    }

    fn aspect_three_two(&mut self) -> &mut Self {
        self.aspect_ratio(Self::THREE_TWO);
        self
    }

    fn aspect_four_three(&mut self) -> &mut Self {
        self.aspect_ratio(Self::FOUR_THREE);
        self
    }

    fn aspect_sixteen_nine(&mut self) -> &mut Self {
        self.aspect_ratio(Self::SIXTEEN_NINE);
        self
    }

    fn aspect_nine_sixteen(&mut self) -> &mut Self {
        self.aspect_ratio(Self::NINE_SIXTEEN);
        self
    }
}
