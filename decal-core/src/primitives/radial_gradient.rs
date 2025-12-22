use crate::paint::ResourceIri;
use crate::primitives::Stop;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct RadialGradient {
    stops: Vec<Stop>,
}

impl RadialGradient {
    pub fn new() -> Self {
        RadialGradient { stops: Vec::new() }
    }
}

impl ResourceIri for RadialGradient {}

impl Display for RadialGradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"<radialGradient id={}"#, self.iri())?;

        if self.stops.is_empty() {
            write!(f, ">")
        } else {
            for stop in &self.stops {
                write!(f, "{stop}")?;
            }

            write!(f, r#"</radialGradient>"#)
        }
    }
}
