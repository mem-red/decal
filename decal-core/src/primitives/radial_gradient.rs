use crate::paint::{IntoResources, Resource, ResourceIri};
use crate::primitives::GradientTransform;
use crate::primitives::Stop;
use crate::utils::ElementWriter;
use std::fmt::{Display, Formatter};

// TODO

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

impl IntoResources for RadialGradient {
    fn into_resources(self) -> Vec<Resource> {
        vec![self.into()]
    }
}

impl Display for RadialGradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let gradient = ElementWriter::new(f, "radialGradient")?
            .attr("id", (self.iri(),))?
            .write(|out| self.transform.write(out, "gradientTransform"))?;

        if self.stops.is_empty() {
            gradient.close()
        } else {
            gradient
                .content(|out| self.stops.iter().try_for_each(|stop| stop.fmt(out)))?
                .close()
        }
    }
}
