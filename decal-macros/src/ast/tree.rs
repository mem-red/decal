use super::node::Node;
use syn::{
    Result as SynResult,
    parse::{Parse, ParseStream},
};

pub struct DecalTree {
    pub root: Node,
}

impl Parse for DecalTree {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let root = input.parse()?;
        Ok(Self { root })
    }
}
