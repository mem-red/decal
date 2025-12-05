use crate::layout::TextMeta;
use crate::layout::{Node, NodeKind};
use crate::macros::impl_node_methods;
use crate::paint::Appearance;
use crate::prelude::Typography;
use taffy::prelude::*;

#[derive(Debug)]
pub struct Text {
    meta: TextMeta,
    layout: Style,
    visual: Appearance,
    typography: Typography,
}

impl Text {
    pub fn new<S>(content: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            meta: TextMeta::new(content.into()),
            layout: Default::default(),
            visual: Default::default(),
            typography: Default::default(),
        }
    }

    pub fn hidden(&mut self, value: bool) -> &mut Self {
        self.layout.display = if value { Display::None } else { Display::Block };
        self
    }

    pub fn build(&self) -> Node {
        let mut meta = self.meta.to_owned();
        meta.set_typography(self.typography.to_owned());

        Node::new(
            NodeKind::Text(meta),
            self.layout.to_owned(),
            self.visual.to_owned(),
            Some(self.typography.to_owned()),
        )
    }
}

impl_node_methods!(Text, [dimensions, position, self_align, text]);
