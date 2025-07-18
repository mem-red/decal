/// Generates builder-style setter methods for `Option` fields in a node struct.
///
/// For each specified field, this macro creates a setter that accepts values as either the raw type,
/// `Some(value)`, or `None`. This approach enables clear and flexible initialization of optional fields.
///
/// # Parameters
/// - `$node_name`: The target struct name.
/// - A comma-separated list of `field: Type` pairs, corresponding to fields of type `Option<Type>` in the struct.
///
/// # Example
/// ```
/// # #[derive(Default)]
/// # struct Node { width: Option<f32>, height: Option<f32> }
/// impl_node_setters!(Node, {
///     /// Sets the width for the node.
///     width: f32,
///     /// Sets the height for the node.
///     height: f32,
/// });
///
/// let mut node = Node::default();
/// node.width(10.0).height(None);
/// ```
///
/// This macro is private.
macro_rules! impl_node_setters {
    (
        $node_name:ident,
        {
            $(
                $(#[$meta:meta])*
                $field:ident : $ty:ty
            ),* $(,)?
        }
    ) => {
        impl $node_name {
            $(
                $(#[$meta])*
                ///
                /// # Arguments
                /// - `value`: The value to set for this property.
                ///
                /// This method accepts a value in any of the following forms: a raw value, `Some(raw_value)`, or `None`.
                ///
                /// # Returns
                /// Returns a mutable reference to this instance for method chaining.
                pub fn $field<T>(&mut self, value: T) -> &mut Self
                where
                    T: Into<Option<$ty>>,
                {
                    self.$field = value.into();
                    self
                }
            )*
        }
    };
}

pub(crate) use impl_node_setters;
