#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Filter {
    Blur(Blur),
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Blur {}
