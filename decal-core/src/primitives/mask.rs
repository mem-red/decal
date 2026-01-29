use crate::{
    paint::ResourceIri,
    primitives::MaskType,
    utils::{
        ElementWriter,
        IsDefault,
    },
};
use std::fmt::{
    Display,
    Formatter,
};

/// The SVG mask element.
#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub(crate) struct Mask {
    content: String,
    r#type: MaskType,
}

impl Mask {
    /// Builds a [`Mask`] by writing SVG content into a buffer.
    ///
    /// # Arguments
    /// - `write_fn`: The closure used to write SVG content into the mask.
    ///
    /// # Returns
    /// - [`Self`] on success.
    /// - [`std::fmt::Error`] if writing fails.
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

    /// Sets the type of the mask.
    ///
    /// # Arguments
    /// - `value`: The [`MaskType`] value.
    ///
    /// # Returns
    /// [`Self`]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_xml;
    use std::fmt::Write;

    #[test]
    fn renders() {
        let mask = Mask::build(|out| out.write_str("content")).unwrap();
        assert_xml(
            mask.to_string(),
            format!(r#"<mask id="{}">content</mask>"#, mask.iri()),
        );
    }

    #[test]
    fn renders_without_content() {
        let mask = Mask::build(|_| Ok(())).unwrap();
        assert_xml(mask.to_string(), format!(r#"<mask id="{}" />"#, mask.iri()));
    }

    #[test]
    fn renders_with_mask_type() {
        let mask = Mask::build(|_| Ok(())).unwrap().r#type(MaskType::Alpha);
        assert_xml(
            mask.to_string(),
            format!(
                r#"<mask id="{}" mask-type="{}" />"#,
                mask.iri(),
                MaskType::Alpha
            ),
        );
    }
}
