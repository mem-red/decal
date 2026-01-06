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
        let clip_path = ElementWriter::new(f, "clipPath")?.attr("id", (self.iri(),))?;

        if self.0.is_empty() {
            clip_path.close()
        } else {
            clip_path
                .content(|out| out.write_str(self.0.as_str()))?
                .close()
        }
    }
}
