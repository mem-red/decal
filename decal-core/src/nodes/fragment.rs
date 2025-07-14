use crate::layout::{DecalFragment, Renderable};

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
}
