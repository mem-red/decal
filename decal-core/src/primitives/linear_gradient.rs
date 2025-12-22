use crate::paint::ResourceIri;
use crate::primitives::Stop;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub struct LinearGradient {
    stops: Vec<Stop>,
}

impl LinearGradient {
    pub fn new() -> Self {
        LinearGradient { stops: Vec::new() }
    }
}

impl ResourceIri for LinearGradient {}

impl Display for LinearGradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"<linearGradient id={}"#, self.iri())?;

        if self.stops.is_empty() {
            write!(f, ">")
        } else {
            for stop in &self.stops {
                write!(f, "{stop}")?;
            }

            write!(f, r#"</linearGradient>"#)
        }
    }
}
