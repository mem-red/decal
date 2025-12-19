use crate::primitives::ResourceDigest;
use std::fmt::{Display, Formatter};
use strict_num::PositiveF32;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
pub struct LinearGradient {
    start: PositiveF32,
    end: PositiveF32,
}

impl LinearGradient {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ResourceDigest for LinearGradient {}

impl Display for LinearGradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<linearGradient id="{}">
                <stop offset="{}%" stop-color="blue" />
                <stop offset="{}%" stop-color="red" />
            </linearGradient>"#,
            self.digest(),
            self.start,
            self.end
        )
    }
}
