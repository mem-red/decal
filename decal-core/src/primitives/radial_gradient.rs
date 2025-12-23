use crate::paint::ResourceIri;
use crate::primitives::GradientTransform;
use crate::primitives::Stop;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub struct RadialGradient {
    stops: Vec<Stop>,
    transform: GradientTransform,
}

impl RadialGradient {
    pub fn new() -> Self {
        RadialGradient::default()
    }
}

impl ResourceIri for RadialGradient {}

impl Display for RadialGradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"<radialGradient id="{}""#, self.iri())?;
        self.transform.write(f)?;

        if self.stops.is_empty() {
            write!(f, " />")
        } else {
            write!(f, ">")?;

            for stop in &self.stops {
                write!(f, "{stop}")?;
            }

            write!(f, r#"</radialGradient>"#)
        }
    }
}
