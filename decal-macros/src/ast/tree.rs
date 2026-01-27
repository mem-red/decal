use super::node::Node;
use syn::{
    Result as SynResult,
    parse::{
        Parse,
        ParseStream,
    },
};

pub(crate) struct SceneTree {
    pub(crate) root: Node,
}

impl Parse for SceneTree {
    fn parse(input: ParseStream) -> SynResult<Self> {
        Ok(Self {
            root: input.parse()?,
        })
    }
}
