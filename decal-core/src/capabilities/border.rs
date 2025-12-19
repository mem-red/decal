use super::Drawable;

macro_rules! impl_side {
    ($method:ident, $field:ident) => {
        fn $method<T>(mut self, value: T) -> Self
        where
            T: Into<Option<crate::primitives::Length>>,
        {
            self.layout_mut().border.$field = value.into().unwrap_or_default().into();
            self
        }
    };
}

pub trait Border: Drawable {
    fn border_width<T>(mut self, value: T) -> Self
    where
        T: crate::attributes::IntoBorder,
    {
        self.layout_mut().border = value.into_border().unwrap_or_default().into();
        self
    }

    fn border_x_width<T>(mut self, value: T) -> Self
    where
        T: crate::attributes::IntoBorderPair,
    {
        let (left, right) = value.into_border_pair().unwrap_or_default();
        self.layout_mut().border.left = left.into();
        self.layout_mut().border.right = right.into();
        self
    }

    fn border_y_width<T>(mut self, value: T) -> Self
    where
        T: crate::attributes::IntoBorderPair,
    {
        let (top, bottom) = value.into_border_pair().unwrap_or_default();
        self.layout_mut().border.top = top.into();
        self.layout_mut().border.bottom = bottom.into();
        self
    }

    impl_side!(border_top_width, top);
    impl_side!(border_right_width, right);
    impl_side!(border_bottom_width, bottom);
    impl_side!(border_left_width, left);

    fn border<T>(mut self, value: T) -> Self
    where
        T: crate::attributes::IntoPaint,
    {
        self.visual_mut().border = value.into_paint().unwrap_or(crate::primitives::Paint::None);
        self
    }
}
