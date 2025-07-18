use crate::nodes::{Column, Fragment, Root, Row, Snippet, Text};
use atree::{Arena, Token};
use taffy::prelude::*;

pub type NodeId = usize;

pub(crate) trait Renderable {
    fn to_svg(&self) -> &str;
}

fn main() -> Result<(), taffy::TaffyError> {
    let mut taffy: TaffyTree<()> = TaffyTree::new();

    let child = taffy.new_leaf(Style {
        size: Size {
            width: Dimension::from_percent(0.5),
            height: Dimension::AUTO,
        },
        ..Default::default()
    })?;

    let node = taffy.new_with_children(
        Style {
            size: Size {
                width: Dimension::from_length(100.0),
                height: Dimension::from_length(100.0),
            },
            justify_content: Some(JustifyContent::Center),
            ..Default::default()
        },
        &[child],
    )?;

    println!("Compute layout with 100x100 viewport:");
    taffy.compute_layout(
        node,
        Size {
            height: AvailableSpace::Definite(100.0),
            width: AvailableSpace::Definite(100.0),
        },
    )?;
    println!("node: {:#?}", taffy.layout(node)?);
    println!("child: {:#?}", taffy.layout(child)?);

    println!("Compute layout with undefined (infinite) viewport:");
    taffy.compute_layout(node, Size::MAX_CONTENT)?;
    println!("node: {:#?}", taffy.layout(node)?);
    println!("child: {:#?}", taffy.layout(child)?);

    Ok(())
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
    pub style: Style,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind, 
        }
    }
}
