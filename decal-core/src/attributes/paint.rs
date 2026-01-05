use crate::primitives::{
    Color, LinearGradient, Paint, PaintLayer, PaintStack, Pattern, RadialGradient,
};

pub trait IntoPaint {
    fn into_paint(self) -> Paint;
}

impl IntoPaint for Paint {
    #[inline]
    fn into_paint(self) -> Paint {
        self
    }
}

impl<T> IntoPaint for T
where
    T: Into<Color>,
{
    #[inline]
    fn into_paint(self) -> Paint {
        Paint::Color(self.into())
    }
}

impl IntoPaint for LinearGradient {
    #[inline]
    fn into_paint(self) -> Paint {
        Paint::LinearGradient(self.into())
    }
}

impl IntoPaint for RadialGradient {
    #[inline]
    fn into_paint(self) -> Paint {
        Paint::RadialGradient(self.into())
    }
}

impl IntoPaint for Pattern {
    #[inline]
    fn into_paint(self) -> Paint {
        Paint::Pattern(self)
    }
}

//

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
