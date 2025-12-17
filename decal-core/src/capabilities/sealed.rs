pub(crate) mod private {
    use crate::layout::Typography;
    use crate::paint::Appearance;
    use taffy::Style;

    /// Sealed avoids exposing private methods and stops crates
    /// other than decal from implementing any traits that use it.
    /// https://docs.rs/byteorder/1.5.0/src/byteorder/lib.rs.html#169
    pub trait Sealed {
        fn layout(&self) -> &Style;
        fn visual(&self) -> &Appearance;
        fn typography(&self) -> &Typography;
        //
        fn layout_mut(&mut self) -> &mut Style;
        fn visual_mut(&mut self) -> &mut Appearance;
        fn typography_mut(&mut self) -> &mut Typography;
    }
}
