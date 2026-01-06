use crate::paint::ResourceIri;
use crate::utils::ElementWriter;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Pattern(String);

impl Pattern {
    pub(crate) fn new(content: String) -> Self {
        Self(content)
    }
}

impl ResourceIri for Pattern {}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        ElementWriter::new(f, "pattern")?
            .attr("id", (self.iri(),))?
            .content(|out| out.write_str(self.0.as_str()))?
            .close()
    }
}
