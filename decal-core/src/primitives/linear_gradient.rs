use crate::primitives::ResourceDigest;
use derivative::Derivative;
use strict_num::PositiveF32;

#[derive(Derivative)]
#[derivative(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
pub struct LinearGradient {
    start: PositiveF32,
    end: PositiveF32,
    #[derivative(Hash = "ignore")]
    digest: u64,
}

impl LinearGradient {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ResourceDigest for LinearGradient {
    fn digest_mut(&mut self) -> &mut u64 {
        &mut self.digest
    }
}
