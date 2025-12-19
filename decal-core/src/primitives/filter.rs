use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Filter {
    Blur(Blur),
}

impl Display for Filter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Blur {}
