macro_rules! impl_node_builder {
    (
        $node:ty,
        build($this:ident) $build:block
    ) => {
        #[allow(private_interfaces)]
        impl crate::capabilities::Sealed for $node {
            #[inline]
            fn layout(&self) -> &taffy::Style {
                &self.layout
            }

            #[inline]
            fn visual(&self) -> &crate::paint::Appearance {
                &self.visual
            }

            #[inline]
            fn typography(&self) -> &crate::layout::Typography {
                &self.typography
            }

            #[inline]
            fn resources(&self) -> &Vec<crate::paint::Resource> {
                &self.resources
            }

            //

            #[inline]
            fn layout_mut(&mut self) -> &mut taffy::Style {
                &mut self.layout
            }

            #[inline]
            fn visual_mut(&mut self) -> &mut crate::paint::Appearance {
                &mut self.visual
            }

            #[inline]
            fn typography_mut(&mut self) -> &mut crate::layout::Typography {
                &mut self.typography
            }

            #[inline]
            fn resources_mut(&mut self) -> &mut Vec<crate::paint::Resource> {
                &mut self.resources
            }
        }

        impl crate::capabilities::Drawable for $node {
            #[inline]
            fn finish(self) -> crate::layout::Node {
                let $this = self;
                $build
            }
        }
    };
}

pub(crate) use impl_node_builder;
