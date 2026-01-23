use crate::{
    paint::ResourceIri,
    utils::ElementWriter,
};
use std::fmt::{
    Display,
    Formatter,
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub(crate) struct Path(String);

impl Path {
    pub(crate) fn build<F>(write_fn: F) -> Result<Self, std::fmt::Error>
    where
        F: FnOnce(&mut String) -> std::fmt::Result,
    {
        let mut data = String::new();
        write_fn(&mut data)?;
        Ok(Path(data))
    }
}

impl ResourceIri for Path {}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "path")?
            .attr("id", (self.iri(),))?
            .attr("d", self.0.as_str())?
            .close()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_xml;
    use std::fmt::Write;

    #[test]
    fn renders() {
        let path = Path::build(|out| out.write_str("data")).unwrap();
        assert_xml(
            path.to_string(),
            format!(r#"<path id="{}" d="data" />"#, path.iri()),
        );
    }
}
