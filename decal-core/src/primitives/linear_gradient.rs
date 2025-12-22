use crate::paint::ResourceIri;
use crate::prelude::Color;
use crate::primitives::{GradientTransform, Stop};
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
pub struct LinearGradient {
    stops: Vec<Stop>,
    transform: GradientTransform,
}

impl LinearGradient {
    pub fn new() -> Self {
        LinearGradient {
            stops: vec![
                Stop::new()
                    .offset(0.1)
                    .color(Color::parse("red"))
                    .opacity(0.5),
                Stop::new()
                    .offset(0.4)
                    .color(Color::parse("blue"))
                    .opacity(0.5),
                Stop::new()
                    .offset(0.9)
                    .color(Color::parse("yellow"))
                    .opacity(0.5),
            ],
            ..Default::default()
        }
    }
}

impl ResourceIri for LinearGradient {}

impl Display for LinearGradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"<linearGradient id="{}""#, self.iri())?;
        self.transform.write(f)?;

        if self.stops.is_empty() {
            write!(f, " />")
        } else {
            write!(f, ">")?;

            for stop in &self.stops {
                write!(f, "{stop}")?;
            }

            write!(f, r#"</linearGradient>"#)
        }
    }
}
