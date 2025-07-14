use crate::nodes::{Column, Fragment, Root, Row, Snippet, Text};
use atree::{Arena, Token};

pub(crate) trait Renderable {
    fn to_svg(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct Decal {
    arena: Arena<Node>,
    root_token: Token,
}

impl Decal {
    pub fn new(root_node: Root) -> Self {
        let (arena, root_token) = Arena::with_data(Node::new(NodeKind::Root(root_node)));
        Self { arena, root_token }
    }

    pub fn root(&self) -> Token {
        self.root_token
    }

    pub fn append_child(&mut self, under: Token, node: Node) -> Token {
        under.append(self.arena_mut(), node)
    }

    pub fn append_fragment(&mut self, under: Token, fragment: DecalFragment) {
        self.arena
            .copy_and_append_subtree(under, &fragment.arena, fragment.root());
    }

    fn arena_mut(&mut self) -> &mut Arena<Node> {
        &mut self.arena
    }
}

#[derive(Debug, Clone)]
pub struct DecalFragment {
    arena: Arena<Node>,
    root_token: Token,
}

impl DecalFragment {
    pub fn new(node_kind: NodeKind) -> Self {
        let (arena, root_token) = Arena::with_data(Node::new(node_kind));
        Self { arena, root_token }
    }

    pub fn root(&self) -> Token {
        self.root_token
    }

    pub fn append_child(&mut self, under: Token, node: Node) -> Token {
        under.append(self.arena_mut(), node)
    }

    fn arena_mut(&mut self) -> &mut Arena<Node> {
        &mut self.arena
    }
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Root(Root),
    Fragment(Fragment),
    Snippet(Snippet),
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
            NodeKind::Fragment(inner) => inner.to_svg(),
            NodeKind::Snippet(inner) => inner.to_svg(),
        }
    }
}

#[derive(Debug, Clone)]
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
