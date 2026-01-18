use crate::paint::ResourceIri;
use crate::primitives::MaskType;
use crate::utils::{ElementWriter, IsDefault};
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub(crate) struct Mask {
    content: String,
    r#type: MaskType,
}

impl Mask {
    pub(crate) fn build<F>(write_fn: F) -> Result<Self, std::fmt::Error>
    where
        F: FnOnce(&mut String) -> std::fmt::Result,
    {
        let mut content = String::new();
        write_fn(&mut content)?;

        Ok(Mask {
            content,
            ..Default::default()
        })
    }

    pub(crate) fn r#type(mut self, value: MaskType) -> Self {
        self.r#type = value;
        self
    }
}

impl ResourceIri for Mask {}

impl Display for Mask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mask = ElementWriter::new(f, "mask")?
            .attr("id", (self.iri(),))?
            .attr_if("mask-type", (self.r#type,), !self.r#type.is_default())?;

        if self.content.is_empty() {
            mask.close()
        } else {
            mask.content(|out| out.write_str(self.content.as_str()))?
                .close()
        }
    }
}
