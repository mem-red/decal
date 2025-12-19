use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Pattern {}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
