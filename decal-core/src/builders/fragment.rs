use crate::layout::{Node, NodeKind};

#[derive(Debug)]
pub struct Fragment;

impl Fragment {
    pub fn new() -> Node {
        Node::new(NodeKind::Fragment, taffy::Style::default())
    }
}
