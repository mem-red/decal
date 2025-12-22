use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use twox_hash::XxHash3_64;

const PREFIX: &'static str = "dcl";

#[derive(Debug)]
pub(crate) struct Iri(u64);

pub(crate) trait ResourceIri: Hash {
    fn iri(&self) -> Iri {
        let mut hasher = XxHash3_64::with_seed(0);
        self.hash(&mut hasher);
        Iri(hasher.finish())
    }
}

impl Display for Iri {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{PREFIX}-{:x}", self.0)
    }
}
