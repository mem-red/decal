use std::fmt::{
    Display,
    Formatter,
};

/// Stable identifier used to reference nodes within a scene graph.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(u64);

impl NodeId {
    #[allow(dead_code)]
    pub(crate) const fn new(val: u64) -> Self {
        Self(val)
    }
}

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for NodeId {
    #[inline]
    fn from(raw: u64) -> Self {
        Self(raw)
    }
}
impl From<NodeId> for u64 {
    #[inline]
    fn from(id: NodeId) -> Self {
        id.0
    }
}
impl From<usize> for NodeId {
    #[inline]
    fn from(raw: usize) -> Self {
        Self(raw as u64)
    }
}
impl From<NodeId> for usize {
    #[inline]
    fn from(id: NodeId) -> Self {
        id.0 as usize
    }
}

impl From<NodeId> for taffy::NodeId {
    fn from(value: NodeId) -> Self {
        taffy::NodeId::from(value.0)
    }
}
