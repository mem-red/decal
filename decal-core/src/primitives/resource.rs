use crate::primitives::{Filter, LinearGradient, Pattern, RadialGradient};
use std::hash::Hash;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Resource {
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Pattern(Pattern),
    Filter(Filter),
}

pub(crate) trait ResourceDigest: Hash {
    fn digest_mut(&mut self) -> &mut u64;

    fn compute_digest(&mut self) {
        use std::hash::Hasher;
        let mut hasher = twox_hash::XxHash3_64::with_seed(0);
        self.hash(&mut hasher);
        *self.digest_mut() = hasher.finish();
    }
}

impl From<LinearGradient> for Resource {
    #[inline]
    fn from(value: LinearGradient) -> Self {
        Self::LinearGradient(value)
    }
}

impl From<RadialGradient> for Resource {
    #[inline]
    fn from(value: RadialGradient) -> Self {
        Self::RadialGradient(value)
    }
}

impl From<Pattern> for Resource {
    #[inline]
    fn from(value: Pattern) -> Self {
        Self::Pattern(value)
    }
}

impl From<Filter> for Resource {
    #[inline]
    fn from(value: Filter) -> Self {
        Self::Filter(value)
    }
}
