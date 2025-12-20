use crate::primitives::ResourceIri;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct RadialGradient {}

impl ResourceIri for RadialGradient {}

impl Display for RadialGradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<radialGradient id="{}">
                <stop offset="5%" stop-color="blue" />
                <stop offset="80%" stop-color="red" />
            </radialGradient>"#,
            self.iri(),
        )
    }
}
