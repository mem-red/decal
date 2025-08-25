use crate::{
    builders::{RootMeta, TextMeta},
    prelude::ImageMeta,
};
use std::fmt::Write;
use taffy::{Cache, prelude::*};

#[derive(Debug, Clone)]
pub(crate) enum NodeKind {
    Root(RootMeta),
    Block,
    Flex,
    Column,
    Row,
    Grid,
    Text(TextMeta),
    Image(ImageMeta),
}

impl NodeKind {
    fn is_atomic(&self) -> bool {
        match self {
            NodeKind::Root(_)
            | NodeKind::Block
            | NodeKind::Flex
            | NodeKind::Column
            | NodeKind::Row
            | NodeKind::Grid => false,
            NodeKind::Text(_) | NodeKind::Image(_) => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) kind: NodeKind,
    pub(crate) style: Style,
    pub(crate) atomic: bool,
    pub(crate) children: Vec<usize>,
    // Computed
    pub(crate) cache: Cache,
    pub(crate) unrounded_layout: Layout,
    pub(crate) final_layout: Layout,
}

impl Node {
    pub(crate) fn new(kind: NodeKind, style: Style) -> Self {
        Self {
            atomic: kind.is_atomic(),
            kind,
            style,
            children: Vec::new(),
            cache: Cache::new(),
            unrounded_layout: Layout::with_order(0),
            final_layout: Layout::with_order(0),
        }
    }

    pub(crate) fn write_svg_start(&self, out: &mut String) {
        match &self.kind {
            NodeKind::Root(_) => {}
            NodeKind::Block | NodeKind::Flex | NodeKind::Column | NodeKind::Row => {
                write!(
                    out,
                    r#"<g class="block"> <rect width="{}" height="{}" fill="lightblue" />"#,
                    self.final_layout.size.width, self.final_layout.size.height,
                )
                .unwrap();
            }
            NodeKind::Text(meta) => {
                let ctx = &meta.context;
                ctx.write_vertorized_text(
                    (self.final_layout.location.x, self.final_layout.location.y),
                    out,
                );

                // write!(
                //     out,
                //     r#"<text x="{}" y="{}">{}</text>"#,
                //     self.final_layout.location.x, self.final_layout.location.y, meta.content,
                // )
                // .unwrap();
            }
            NodeKind::Image(meta) => {
                write!(
                    out,
                    r#"<image href="{}" x="{}" y="{}" width="{}" height="{}"/>"#,
                    meta.source,
                    self.final_layout.location.x,
                    self.final_layout.location.y,
                    self.final_layout.size.width,
                    self.final_layout.size.height,
                )
                .unwrap();
            }
            _ => {}
        }
    }

    pub(crate) fn write_svg_end(&self, out: &mut String) {
        match &self.kind {
            NodeKind::Block
            | NodeKind::Flex
            | NodeKind::Column
            | NodeKind::Row
            | NodeKind::Grid => out.push_str("</g>"),
            NodeKind::Root(_) | NodeKind::Text(_) | NodeKind::Image(_) => {}
        }
    }
}
