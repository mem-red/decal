use super::Drawable;
use crate::{
    attributes::{
        IntoBorder,
        IntoBorderPair,
        IntoPaintStack,
    },
    primitives::IntoOptionalLength,
};

macro_rules! impl_side {
    ($method:ident, $field:ident) => {
        fn $method<T>(mut self, value: T) -> Self
        where
            T: IntoOptionalLength<false, true>,
        {
            self.layout_mut().border.$field =
                value.into_optional_length().unwrap_or_default().into();
            self
        }
    };
}

pub trait Border: Drawable {
    fn border_width<T>(mut self, value: T) -> Self
    where
        T: IntoBorder,
    {
        self.layout_mut().border = value.into_border().unwrap_or_default().into();
        self
    }

    fn border_x_width<T>(mut self, value: T) -> Self
    where
        T: IntoBorderPair,
    {
        let (left, right) = value.into_border_pair().unwrap_or_default();
        self.layout_mut().border.left = left.into();
        self.layout_mut().border.right = right.into();
        self
    }

    fn border_y_width<T>(mut self, value: T) -> Self
    where
        T: IntoBorderPair,
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
        T: IntoPaintStack,
    {
        let border = value.into_paint_stack();
        self.visual_mut().border = border.clone();
        self.add_resources(border);

        self
    }
}
