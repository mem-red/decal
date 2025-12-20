use crate::primitives::ResourceIri;
use std::fmt::{Display, Formatter};
use strict_num::PositiveF32;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
pub struct LinearGradient {
    start: PositiveF32,
    end: PositiveF32,
}

impl LinearGradient {
    pub fn new(start: f32, end: f32) -> Self {
        LinearGradient {
            start: PositiveF32::new(start).unwrap_or_default(),
            end: PositiveF32::new(end).unwrap_or_default(),
        }
    }
}

impl ResourceIri for LinearGradient {}

impl Display for LinearGradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<linearGradient id="{}">
                <stop offset="{}%" stop-color="blue" />
                <stop offset="{}%" stop-color="red" />
            </linearGradient>"#,
            self.iri(),
            self.start,
            self.end
        )
    }
}
