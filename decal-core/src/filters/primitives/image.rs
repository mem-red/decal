use crate::{
    filters::{
        FilterRegion,
        HasFilterRegion,
        primitives::PrimitiveBuilder,
    },
    paint::ResourceIri,
    primitives::CrossOrigin,
    utils::ElementWriter,
};
use std::fmt::{
    Display,
    Formatter,
};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Image {
    href: String,
    cross_origin: Option<CrossOrigin>,
    region: FilterRegion,
}

impl Image {
    pub(crate) fn new(href: &str) -> Self {
        Image {
            href: href.to_string(),
            cross_origin: None,
            region: Default::default(),
        }
    }
}

impl HasFilterRegion for Image {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl ResourceIri for Image {}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "feImage")?
            .write(|out| self.region.fmt(out))?
            .attr("href", self.href.as_str())?
            .attr("crossorigin", self.cross_origin.map(|x| (x,)))?
            .attr("result", (self.iri(),))?
            .close()
    }
}

impl<'a> PrimitiveBuilder<'a, Image> {
    pub fn cross_origin<T>(mut self, cross_origin: T) -> Self
    where
        T: Into<Option<CrossOrigin>>,
    {
        self.inner.cross_origin = cross_origin.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        filters::{
            FilterContext,
            FilterRegionConfig,
        },
        test_utils::assert_xml,
    };

    #[test]
    fn renders_with_filter_region() {
        let ctx = FilterContext::default();
        ctx.image("test")
            .x(0.5)
            .y(0.6)
            .width(110)
            .height(120)
            .finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feImage
    x="0.5"
    y="0.6"
    width="110"
    height="120"
    href="test"
    result="{}"
/>
"#,
                node.iri()
            ),
        );
    }

    #[test]
    fn renders() {
        let ctx = FilterContext::default();
        ctx.image("test").finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(r#"<feImage href="test" result="{}" />"#, node.iri()),
        );
    }

    #[test]
    fn renders_with_attrs() {
        let ctx = FilterContext::default();
        let cross_origin = CrossOrigin::UseCredentials;
        ctx.image("test").cross_origin(cross_origin).finish();
        let node = &ctx.into_primitives()[0];

        assert_xml(
            node.to_string(),
            format!(
                r#"
<feImage
    href="test"
    crossorigin="{cross_origin}"
    result="{}"
/>
"#,
                node.iri()
            ),
        );
    }
}
