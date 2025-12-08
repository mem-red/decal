use crate::layout::text::TextMeta;
use crate::layout::{FontRegistry, TextVectorizationError};
use crate::paint::{Appearance, compute_scaled_radii};
use crate::paint::{ScaledRadii, write_border_path, write_clip_path, write_fill_path};
use crate::prelude::Typography;
use crate::{builders::RootMeta, prelude::ImageMeta};
use std::fmt::{Display, Write};
use std::sync::{Arc, Mutex};
use taffy::{Cache, prelude::*};
use thiserror::Error;

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
    pub(crate) fn is_atomic(&self) -> bool {
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

impl Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NodeKind::Root(meta) => format!("Root: {meta:?}"),
                NodeKind::Block => "Block".into(),
                NodeKind::Flex => "Flex".into(),
                NodeKind::Column => "Column".into(),
                NodeKind::Row => "Row".into(),
                NodeKind::Grid => "Grid".into(),
                NodeKind::Text(TextMeta { spans, .. }) => format!(
                    "Text: {}",
                    spans
                        .iter()
                        .map(|x| x.content.clone())
                        .collect::<Vec<_>>()
                        .join("")
                ),
                NodeKind::Image(meta) => format!("Image: {meta:?}"),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) kind: NodeKind,
    pub(crate) layout: Style,
    pub(crate) visual: Appearance,
    pub(crate) children: Vec<usize>,
    pub(crate) typography: Typography,
    // computed
    pub(crate) cache: Cache,
    pub(crate) unrounded_layout: Layout,
    pub(crate) final_layout: Layout,
    pub(crate) scaled_radii: ScaledRadii,
}

#[derive(Debug, Error)]
pub enum VectorizationError {
    #[error("cannot write to stream")]
    SvgWrite(#[from] std::fmt::Error),
    #[error("text vectorization error")]
    Text(#[from] TextVectorizationError),
    #[error("other error")]
    Other,
}

impl Node {
    pub(crate) fn new(
        kind: NodeKind,
        layout: Style,
        visual: Appearance,
        typography: Option<Typography>,
    ) -> Self {
        Self {
            kind,
            layout,
            visual,
            children: Vec::new(),
            typography: typography.unwrap_or(Typography::default()),
            cache: Cache::new(),
            unrounded_layout: Layout::with_order(0),
            final_layout: Layout::with_order(0),
            scaled_radii: ScaledRadii::default(),
        }
    }

    pub(crate) fn apply_layout_effects(&mut self) {
        self.scaled_radii = compute_scaled_radii(
            self.visual.corner_radius,
            self.final_layout.size.width,
            self.final_layout.size.height,
        );
    }

    pub(crate) fn write_svg_start<T>(
        &self,
        out: &mut T,
        root_size: (f32, f32),
        fonts: Arc<Mutex<FontRegistry>>,
        node_id: usize,
    ) -> Result<(), VectorizationError>
    where
        T: Write,
    {
        match &self.kind {
            NodeKind::Root(_) => {}
            //
            NodeKind::Block
            | NodeKind::Flex
            | NodeKind::Column
            | NodeKind::Row
            | NodeKind::Grid => {
                let w = self.final_layout.size.width;
                let h = self.final_layout.size.height;
                let radius = self.scaled_radii;
                let borders = (
                    self.final_layout.border.top,
                    self.final_layout.border.right,
                    self.final_layout.border.bottom,
                    self.final_layout.border.left,
                );
                let clip_x = self.layout.overflow.x == taffy::Overflow::Hidden;
                let clip_y = self.layout.overflow.y == taffy::Overflow::Hidden;
                let use_clip = clip_x || clip_y;

                write!(out, r#"<g"#)?;
                self.visual.transform.write_transform_matrix(
                    out,
                    (0.0, 0.0),
                    (self.final_layout.location.x, self.final_layout.location.y),
                    (w, h),
                )?;
                write!(out, r#">"#)?;

                // background
                write!(out, r#"<path d=""#)?;
                write_fill_path(out, w, h, radius)?;
                write!(out, r#"" fill="{}"/>"#, self.visual.background)?;

                // borders
                if borders.0 + borders.1 + borders.2 + borders.3 > 0.0 {
                    write!(out, r#"<path d=""#)?;
                    write_border_path(
                        out,
                        self.final_layout.size.width,
                        self.final_layout.size.height,
                        radius,
                        borders,
                    )?;
                    write!(
                        out,
                        r#"" fill="{}" fill-rule="evenodd" clip-rule="evenodd" />"#,
                        self.visual.border_color
                    )?;
                }

                // clip content
                if use_clip {
                    let clip_id = format!("decal-clip-{node_id}");
                    write!(out, r#"<clipPath id="{clip_id}"><path d=""#)?;
                    write_clip_path(out, w, h, radius, borders, clip_x, clip_y, root_size)?;
                    write!(out, r#""/></clipPath>"#)?;
                    write!(out, r#"<g clip-path="url(#{clip_id})">"#)?;
                }
            }
            //
            NodeKind::Text(meta) => {
                let mut fonts = fonts.lock().map_err(|_| VectorizationError::Other)?;
                meta.write_vectorized_text(
                    out,
                    (self.final_layout.location.x, self.final_layout.location.y),
                    &self.visual.transform,
                    &mut fonts.system,
                )?;
            }
            //
            NodeKind::Image(meta) => {
                // TODO fix and write transform
                write!(
                    out,
                    r#"<image href="{}" x="{}" y="{}" width="{}" height="{}"/>"#,
                    meta.source,
                    self.final_layout.location.x,
                    self.final_layout.location.y,
                    self.final_layout.size.width,
                    self.final_layout.size.height,
                )?;
            }
        };

        Ok(())
    }

    pub(crate) fn write_svg_end<T>(
        &self,
        out: &mut T,
        _root_size: (f32, f32),
        _node_id: usize,
    ) -> Result<(), VectorizationError>
    where
        T: Write,
    {
        match &self.kind {
            NodeKind::Block
            | NodeKind::Flex
            | NodeKind::Column
            | NodeKind::Row
            | NodeKind::Grid => {
                if self.layout.overflow.x == taffy::Overflow::Hidden
                    || self.layout.overflow.y == taffy::Overflow::Hidden
                {
                    write!(out, r#"</g>"#)?; // close clip group
                }

                write!(out, r#"</g>"#)?; // close transform group
            }
            //
            NodeKind::Root(_) | NodeKind::Text(_) | NodeKind::Image(_) => {}
        }

        Ok(())
    }
}
