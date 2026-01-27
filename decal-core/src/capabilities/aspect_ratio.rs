use super::Drawable;

/// Capability for configuring a fixed aspect ratio on node.
pub trait AspectRatio: Drawable {
    /// Square aspect ratio of 1:1.
    const SQUARE: f32 = 1.0;

    /// A 3:2 aspect ratio.
    const THREE_TWO: f32 = 3.0 / 2.0;

    /// A 4:3 aspect ratio.
    const FOUR_THREE: f32 = 4.0 / 3.0;

    /// A 16:9 aspect ratio.
    const SIXTEEN_NINE: f32 = 16.0 / 9.0;

    /// A 9:16 aspect ratio.
    const NINE_SIXTEEN: f32 = 9.0 / 16.0;

    /// Sets an aspect ratio on the node.
    ///
    /// # Arguments
    /// - `value`: The aspect ratio expressed as width divided by height.
    ///
    /// # Returns
    /// - [`Self`]
    fn aspect_ratio<T>(mut self, value: T) -> Self
    where
        T: Into<Option<f32>>,
    {
        self.layout_mut().aspect_ratio = value.into();
        self
    }

    /// Sets the aspect ratio to `1:1`.
    ///
    /// # Returns
    /// - [`Self`]
    fn aspect_square(self) -> Self {
        self.aspect_ratio(Self::SQUARE)
    }

    /// Sets the aspect ratio to `3:2`.
    ///
    /// # Returns
    /// - [`Self`]
    fn aspect_three_two(self) -> Self {
        self.aspect_ratio(Self::THREE_TWO)
    }

    /// Sets the aspect ratio to `4:3`.
    ///
    /// # Returns
    /// - [`Self`]
    fn aspect_four_three(self) -> Self {
        self.aspect_ratio(Self::FOUR_THREE)
    }

    /// Sets the aspect ratio to `16:9`.
    ///
    /// # Returns
    /// - [`Self`]
    fn aspect_sixteen_nine(self) -> Self {
        self.aspect_ratio(Self::SIXTEEN_NINE)
    }

    /// Sets the aspect ratio to `9:16`.
    ///
    /// # Returns
    /// - [`Self`]
    fn aspect_nine_sixteen(self) -> Self {
        self.aspect_ratio(Self::NINE_SIXTEEN)
    }
}
