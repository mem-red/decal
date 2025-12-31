use crate::layout::text::TextMeta;
use crate::layout::{FontRegistry, ImageSource, SvgDimensions, TextVectorizeError};
use crate::layout::{Typography, VectorizeOptions};
use crate::paint::{Appearance, Resources, compute_scaled_radii};
use crate::paint::{Resource, ResourceIri};
use crate::paint::{ScaledRadii, write_border_path, write_clip_path, write_fill_path};
use crate::primitives::ClipPath;
use crate::utils::IsDefault;
use crate::{builders::RootMeta, prelude::ImageMeta};
use enum_display::EnumDisplay;
use std::fmt::Write;
use std::sync::{Arc, Mutex};
use taffy::{Cache, Point, Size, prelude::*};
use thiserror::Error;

#[derive(Debug, Clone, EnumDisplay)]
pub(crate) enum NodeKind {
    #[display("Root: {0:?}")]
    Root(RootMeta),
    Block,
    Flex,
    Column,
    Row,
    Grid,
    #[display("Text: {0}")]
    Text(TextMeta),
    #[display("Image: {0:?}")]
    Image(ImageMeta),
}

impl NodeKind {
    pub(crate) fn is_atomic(&self) -> bool {
        matches!(self, NodeKind::Text(_) | NodeKind::Image(_))
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) kind: NodeKind,
    pub(crate) layout: Style,
    pub(crate) visual: Appearance,
    pub(crate) children: Vec<usize>,
    pub(crate) resources: Vec<Resource>,
    pub(crate) typography: Typography,
    // computed
    pub(crate) cache: Cache,
    pub(crate) unrounded_layout: Layout,
    pub(crate) final_layout: Layout,
    pub(crate) scaled_radii: ScaledRadii,
}

#[derive(Debug, Error)]
pub enum VectorizeError {
    #[error("cannot vectorize a fragment")]
    NonRootNode,
    #[error("failed to write to the output stream")]
    Write(#[from] std::fmt::Error),
    #[error("failed to vectorize text")]
    TextVectorize(#[from] TextVectorizeError),
    #[error("internal error")]
    Other,
}

impl Node {
    pub(crate) fn new(
        kind: NodeKind,
        layout: Style,
        visual: Appearance,
        typography: Option<Typography>,
        resources: Vec<Resource>,
    ) -> Self {
        Self {
            kind,
            layout,
            visual,
            children: Vec::new(),
            resources,
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
        resources: &Mutex<Resources>,
        options: &VectorizeOptions,
    ) -> Result<Option<Vec<Resource>>, VectorizeError>
    where
        T: Write,
    {
        match &self.kind {
            NodeKind::Root(meta) => {
                let (w, h) = (meta.width, meta.height);

                write!(out, "<svg")?;

                if !options.omit_svg_xmlns {
                    write!(out, r#" xmlns="http://www.w3.org/2000/svg""#)?;
                }

                write!(out, r#" viewBox="0 0 {w} {h}""#)?;

                match &options.svg_dimensions {
                    SvgDimensions::Omit => {}
                    SvgDimensions::Layout => write!(out, r#" width="{w}" height="{h}""#)?,
                    SvgDimensions::Custom { width, height } => {
                        write!(out, r#" width="{width}" height="{height}""#)?
                    }
                };

                out.write_char('>')?;
            }
            //
            NodeKind::Block
            | NodeKind::Flex
            | NodeKind::Column
            | NodeKind::Row
            | NodeKind::Grid => {
                let Size {
                    width: w,
                    height: h,
                } = self.final_layout.size;
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

                write!(out, "<g")?;

                if self.visual.opacity != 1.0 {
                    write!(out, r#" opacity="{}""#, self.visual.opacity)?;
                }

                if !self.visual.filter.is_default() {
                    write!(out, r#" filter="url(#{})""#, self.visual.filter.iri())?;
                }

                self.visual.transform.write(
                    out,
                    (0.0, 0.0),
                    (self.final_layout.location.x, self.final_layout.location.y),
                    (w, h),
                )?;

                write!(out, ">")?;

                // background
                if !self.visual.background.is_none() {
                    write!(out, r#"<path d=""#)?;
                    write_fill_path(out, w, h, radius)?;
                    write!(out, r#"""#)?;
                    write!(out, r#" fill="{}""#, self.visual.background)?;

                    if self.visual.background_opacity != 1.0 {
                        write!(out, r#" fill-opacity="{}""#, self.visual.background_opacity)?;
                    }

                    write!(out, " />")?;
                }

                // borders
                if borders.0 + borders.1 + borders.2 + borders.3 > 0.0 {
                    write!(out, r#"<path d=""#)?;
                    write_border_path(out, w, h, radius, borders)?;
                    write!(
                        out,
                        r#"" fill="{}" fill-rule="evenodd" clip-rule="evenodd" />"#,
                        self.visual.border
                    )?;
                }

                // clip content
                if use_clip {
                    let clip = ClipPath::new({
                        let mut content = String::new();
                        write!(content, r#"<path d=""#)?;
                        write_clip_path(
                            &mut content,
                            w,
                            h,
                            radius,
                            borders,
                            clip_x,
                            clip_y,
                            root_size,
                        )?;
                        write!(content, r#"" />"#)?;
                        content
                    });

                    write!(out, r#"<g clip-path="url(#{})">"#, clip.iri())?;
                    resources
                        .lock()
                        .map_err(|_| VectorizeError::Other)?
                        .get_or_add_resource(clip.into());
                }
            }
            //
            NodeKind::Text(meta) => {
                let mut fonts = fonts.lock().map_err(|_| VectorizeError::Other)?;
                let FontRegistry {
                    swash_cache,
                    system,
                    ..
                } = &mut *fonts;

                meta.vectorize_text(
                    out,
                    (self.final_layout.location.x, self.final_layout.location.y),
                    &self.visual,
                    swash_cache,
                    system,
                )?;
            }
            //
            NodeKind::Image(meta) => {
                let Point { x, y } = self.final_layout.location;
                let Size {
                    width: w,
                    height: h,
                } = self.final_layout.size;

                match &meta.source {
                    ImageSource::Url(_) | ImageSource::DataUri(_) => {
                        write!(
                            out,
                            r#"<image href="{}" x="{x}" y="{y}" width="{w}" height="{h}""#,
                            meta.source,
                        )?;

                        if self.visual.opacity != 1.0 {
                            write!(out, r#" opacity="{}""#, self.visual.opacity)?;
                        }

                        if !self.visual.filter.is_default() {
                            write!(out, r#" filter="url(#{})""#, self.visual.filter.iri())?;
                        }

                        self.visual
                            .transform
                            .write(out, (x, y), (0.0, 0.0), (w, h))?;

                        if let Some(cross_origin) = meta.cross_origin {
                            write!(out, r#" crossorigin="{cross_origin}""#)?;
                        }

                        write!(out, " />")?;
                    }

                    ImageSource::Svg(svg) => {
                        write!(out, "<g")?;

                        if self.visual.opacity != 1.0 {
                            write!(out, r#" opacity="{}""#, self.visual.opacity)?;
                        }

                        if !self.visual.filter.is_default() {
                            write!(out, r#" filter="url(#{})""#, self.visual.filter.iri())?;
                        }

                        self.visual
                            .transform
                            .write(out, (0.0, 0.0), (x, y), (w, h))?;

                        write!(out, ">")?;

                        out.write_str(svg)?;

                        write!(out, "</g>")?;
                    }
                };
            }
        };

        Ok(None)
    }

    pub(crate) fn write_svg_end<T>(
        &self,
        out: &mut T,
        resources: &Mutex<Resources>,
    ) -> Result<(), VectorizeError>
    where
        T: Write,
    {
        match &self.kind {
            NodeKind::Root(_) => {
                let resources = resources.lock().map_err(|_| VectorizeError::Other)?;

                if !resources.is_empty() {
                    write!(out, "<defs>{resources}</defs>")?;
                }

                write!(out, "</svg>")?;
            }
            //
            NodeKind::Block
            | NodeKind::Flex
            | NodeKind::Column
            | NodeKind::Row
            | NodeKind::Grid => {
                if self.layout.overflow.x == taffy::Overflow::Hidden
                    || self.layout.overflow.y == taffy::Overflow::Hidden
                {
                    write!(out, "</g>")?; // close clip group
                }

                write!(out, "</g>")?; // close transform group
            }
            //
            NodeKind::Text(_) | NodeKind::Image(_) => {}
        }

        Ok(())
    }
}
