use crate::paint::ResourceIri;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub(crate) struct ClipPath(String);

impl ClipPath {
    pub(crate) fn new(content: String) -> Self {
        ClipPath(content)
    }
}

impl ResourceIri for ClipPath {}

impl Display for ClipPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"<clipPath id="{}""#, self.iri())?;

        if self.0.is_empty() {
            write!(f, " />")
        } else {
            write!(f, r#">{}</clipPath>"#, self.0)
        }
    }
}
