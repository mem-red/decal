use crate::nodes::{Column, Root, Row, Text};
use atree::{Arena, Token};

pub(crate) trait Renderable {
    fn to_svg(&self) -> &str;
}

#[derive(Debug)]
pub struct Decal {
    pub arena: Arena<Node>,
}

impl Decal {
    pub fn new(root_node: Root) -> (Self, Token) {
        let (arena, root_tkn) = Arena::with_data(Node::new(NodeKind::Root(root_node)));
        (Self { arena }, root_tkn)
    }

    pub fn arena_mut(&mut self) -> &mut Arena<Node> {
        &mut self.arena
    }
}

#[derive(Debug)]
pub struct DecalPartial {
    pub arena: Arena<Node>,
}

impl DecalPartial {
    pub fn new(node_kind: NodeKind) -> (Self, Token) {
        let (arena, root_tkn) = Arena::with_data(Node::new(node_kind));
        (Self { arena }, root_tkn)
    }

    pub fn arena_mut(&mut self) -> &mut Arena<Node> {
        &mut self.arena
    }
}

#[derive(Debug)]
pub enum NodeKind {
    Root(Root),
    Column(Column),
    Row(Row),
    Text(Text),
}

impl Renderable for Node {
    fn to_svg(&self) -> &str {
        match &self.kind {
            NodeKind::Root(inner) => inner.to_svg(),
            NodeKind::Column(inner) => inner.to_svg(),
            NodeKind::Row(inner) => inner.to_svg(),
            NodeKind::Text(inner) => inner.to_svg(),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    // Cache
    pub(crate) computed_width: Option<f32>,
    pub(crate) computed_height: Option<f32>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind,
            computed_height: None,
            computed_width: None,
        }
    }
}
