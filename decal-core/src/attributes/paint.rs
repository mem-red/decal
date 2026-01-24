use crate::primitives::{
    PaintLayer,
    PaintStack,
};

pub trait IntoPaintLayer {
    fn into_layer(self) -> PaintLayer;
}

impl<T> IntoPaintLayer for T
where
    T: Into<PaintLayer>,
{
    #[inline]
    fn into_layer(self) -> PaintLayer {
        self.into()
    }
}

//

pub trait IntoPaintStack {
    fn into_paint_stack(self) -> PaintStack;
}

impl<T> IntoPaintStack for T
where
    T: IntoPaintLayer,
{
    #[inline]
    fn into_paint_stack(self) -> PaintStack {
        PaintStack::new(vec![self.into_layer()])
    }
}

// allow `None` for default paint stack
impl IntoPaintStack for Option<()> {
    #[inline]
    fn into_paint_stack(self) -> PaintStack {
        PaintStack::default()
    }
}

impl<T, const N: usize> IntoPaintStack for [T; N]
where
    T: IntoPaintLayer,
{
    #[inline]
    fn into_paint_stack(self) -> PaintStack {
        PaintStack::new(self.into_iter().map(IntoPaintLayer::into_layer))
    }
}

impl<T> IntoPaintStack for Vec<T>
where
    T: IntoPaintLayer,
{
    #[inline]
    fn into_paint_stack(self) -> PaintStack {
        PaintStack::new(self.into_iter().map(IntoPaintLayer::into_layer))
    }
}
