use crate::primitives::{
    PaintLayer,
    PaintStack,
};

/// Conversion trait for values that can be interpreted as a paint layer.
pub trait IntoPaintLayer {
    /// Converts the value into a single paint layer.
    fn into_layer(self) -> PaintLayer;
}

/// Blanket implementation for types that can be directly converted into a paint
/// layer.
impl<T> IntoPaintLayer for T
where
    T: Into<PaintLayer>,
{
    /// Delegates to the underlying `Into<PaintLayer>` implementation.
    #[inline]
    fn into_layer(self) -> PaintLayer {
        self.into()
    }
}

/// Conversion trait for values that can be interpreted as a paint stack.
///
/// # Examples
///
/// Paint stack from simple paint primitives.
///
/// ```rust
/// # use decal::prelude::*;
///
/// let gradients: PaintStack = [
///     LinearGradient::angle(-30.0).stops([(0.0, rgb(0x0000ff)), (1.0, rgba(0x0000ff00))]),
///     LinearGradient::angle(180.0).stops([(0.0, rgb(0x00ff00)), (1.0, rgba(0x00ff0000))]),
/// ]
/// .into_paint_stack();
/// ```
///
/// Paint stack from paint layers with opacity and blend modes.
///
/// ```rust
/// # use decal::prelude::*;
///
/// let complex_stack: PaintStack = [
///     Color::rgb(25, 25, 25).into_layer(),
///     ImagePaint::new("<image-href>")
///         .top_right()
///         .into_layer()
///         .blend_mode(BlendMode::Hue)
///         .opacity(0.5),
///     LinearGradient::bottom()
///         .stops([(0.0, rgb(0x0000ff)), (1.0, rgba(0x0000ff00))])
///         .into_layer()
///         .blend_mode(BlendMode::Exclusion),
/// ]
/// .into_paint_stack();
/// ```
pub trait IntoPaintStack {
    /// Converts the value into a paint stack.
    fn into_paint_stack(self) -> PaintStack;
}

/// Converts a single paint layer into a paint stack containing one layer.
impl<T> IntoPaintStack for T
where
    T: IntoPaintLayer,
{
    /// Wraps the converted paint layer into a new paint stack.
    #[inline]
    fn into_paint_stack(self) -> PaintStack {
        PaintStack::new(vec![self.into_layer()])
    }
}

/// Allows `None` to represent a default paint stack.
impl IntoPaintStack for Option<()> {
    /// Returns the default paint stack.
    #[inline]
    fn into_paint_stack(self) -> PaintStack {
        PaintStack::default()
    }
}

/// Converts an array of paint layers into a paint stack.
impl<T, const N: usize> IntoPaintStack for [T; N]
where
    T: IntoPaintLayer,
{
    /// Converts each array element into a paint layer and collects them into a
    /// paint stack.
    #[inline]
    fn into_paint_stack(self) -> PaintStack {
        PaintStack::new(self.into_iter().map(IntoPaintLayer::into_layer))
    }
}

/// Converts a vector of paint layers into a paint stack.
impl<T> IntoPaintStack for Vec<T>
where
    T: IntoPaintLayer,
{
    /// Converts each vector element into a paint layer and collects them into a
    /// paint stack.
    #[inline]
    fn into_paint_stack(self) -> PaintStack {
        PaintStack::new(self.into_iter().map(IntoPaintLayer::into_layer))
    }
}
