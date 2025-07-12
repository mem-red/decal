use crate::layout::{DecalFragment, Node, Renderable};
use atree::Arena;

#[derive(Debug, Clone)]
pub struct Fragment {
    pub fragment: DecalFragment,
}

impl Renderable for Fragment {
    fn to_svg(&self) -> &str {
        todo!()
    }
}

impl Fragment {
    pub fn new(fragment: DecalFragment) -> Self {
        Self { fragment }
    }

    pub fn get_arena(&self) -> &Arena<Node> {
        &self.fragment.arena
    }
}
