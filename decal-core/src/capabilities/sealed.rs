pub(crate) mod private {
    use crate::layout::Typography;
    use crate::paint::Appearance;
    use crate::paint::{IntoResource, Resource};
    use taffy::Style;

    /// Sealed avoids exposing private methods and stops crates
    /// other than decal from implementing any traits that use it.
    /// https://docs.rs/byteorder/1.5.0/src/byteorder/lib.rs.html#169
    pub trait Sealed {
        fn layout(&self) -> &Style;
        fn visual(&self) -> &Appearance;
        fn typography(&self) -> &Typography;
        #[allow(private_interfaces)]
        fn resources(&self) -> &Vec<Resource>;
        //
        fn layout_mut(&mut self) -> &mut Style;
        fn visual_mut(&mut self) -> &mut Appearance;
        fn typography_mut(&mut self) -> &mut Typography;
        #[allow(private_interfaces)]
        fn resources_mut(&mut self) -> &mut Vec<Resource>;
        //
        #[allow(private_bounds)]
        fn add_resource<T>(&mut self, value: T)
        where
            T: IntoResource,
        {
            if let Some(resource) = value.into_resource() {
                self.resources_mut().push(resource);
            }
        }
    }
}
