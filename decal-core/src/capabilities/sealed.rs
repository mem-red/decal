pub(crate) mod private {
    use crate::{
        layout::Typography,
        paint::{
            Appearance,
            IntoResources,
            Resource,
        },
    };
    use taffy::Style;

    /// Sealed avoids exposing private methods and stops crates
    /// other than decal from implementing any traits that use it.
    /// https://docs.rs/byteorder/1.5.0/src/byteorder/lib.rs.html#169
    pub trait Sealed {
        /// Returns a reference to the node layout.
        fn layout(&self) -> &Style;

        /// Returns a reference to the node visual appearance.
        fn visual(&self) -> &Appearance;

        /// Returns a reference to the node typography.
        #[allow(private_interfaces)]
        fn typography(&self) -> &Typography;

        /// Returns a reference to the resources associated with the node.
        #[allow(private_interfaces)]
        fn resources(&self) -> &Vec<Resource>;

        /// Returns a mutable reference to the node layout.
        fn layout_mut(&mut self) -> &mut Style;

        /// Returns a mutable reference to the node visual appearance.
        fn visual_mut(&mut self) -> &mut Appearance;

        /// Returns a mutable reference to the node typography.
        #[allow(private_interfaces)]
        fn typography_mut(&mut self) -> &mut Typography;

        /// Returns a mutable reference to the resources associated with the
        /// node.
        #[allow(private_interfaces)]
        fn resources_mut(&mut self) -> &mut Vec<Resource>;

        /// Adds resources derived from the provided value to the node.
        ///
        /// # Arguments
        /// - `value`: The resource source convertible using [`IntoResources`].
        #[allow(private_bounds)]
        fn add_resources<T>(&mut self, value: T)
        where
            T: IntoResources,
        {
            self.resources_mut().extend(value.into_resources());
        }
    }
}
