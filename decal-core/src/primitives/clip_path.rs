use crate::{
    paint::ResourceIri,
    utils::ElementWriter,
};
use std::fmt::{
    Display,
    Formatter,
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub(crate) struct ClipPath(String);

impl ClipPath {
    pub(crate) fn build<F>(write_fn: F) -> Result<Self, std::fmt::Error>
    where
        F: FnOnce(&mut String) -> std::fmt::Result,
    {
        let mut data = String::new();
        write_fn(&mut data)?;
        Ok(ClipPath(data))
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
