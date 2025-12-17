use super::Drawable;

macro_rules! impl_dimension {
    ($method:ident, $taffy_method:ident, $taffy_field:ident) => {
        fn $method<T>(&mut self, value: T) -> &mut Self
        where
            T: Into<Option<crate::primitives::Length>>,
        {
            self.layout_mut().$taffy_method.$taffy_field = value
                .into()
                .map_or(taffy::Dimension::auto(), |inner| inner.into());
            self
        }
    };
}

pub trait Dimensions: Drawable {
    fn size<T>(&mut self, value: T) -> &mut Self
    where
        T: crate::attributes::IntoDimensions,
    {
        self.layout_mut().size = value
            .into_dimensions()
            .map_or(taffy::Size::auto(), |inner| inner.into());
        self
    }

    fn min_size<T>(&mut self, value: T) -> &mut Self
    where
        T: crate::attributes::IntoDimensions,
    {
        self.layout_mut().min_size = value
            .into_dimensions()
            .map_or(taffy::Size::auto(), |inner| inner.into());
        self
    }

    fn max_size<T>(&mut self, value: T) -> &mut Self
    where
        T: crate::attributes::IntoDimensions,
    {
        self.layout_mut().max_size = value
            .into_dimensions()
            .map_or(taffy::Size::auto(), |inner| inner.into());
        self
    }

    impl_dimension!(width, size, width);
    impl_dimension!(height, size, height);
    //
    impl_dimension!(min_width, min_size, width);
    impl_dimension!(min_height, min_size, height);
    //
    impl_dimension!(max_width, max_size, width);
    impl_dimension!(max_height, max_size, height);
}
