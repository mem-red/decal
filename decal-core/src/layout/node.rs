use crate::builders::RootMeta;
use crate::layout::text::TextMeta;
use crate::layout::{FontRegistry, ImageSource, RenderContext, SvgDimensions, TextVectorizeError};
use crate::layout::{ImageMeta, Typography};
use crate::paint::{Appearance, compute_scaled_radii};
use crate::paint::{Resource, ResourceIri};
use crate::paint::{ScaledRadii, write_border_path, write_clip_path, write_fill_path};
use crate::primitives::{ClipPath, Path, Rect};
use crate::utils::{ElementWriter, IsDefault};
use enum_display::EnumDisplay;
use std::fmt::Write;
use strict_num::NormalizedF32;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VectorizeError {
    #[error("cannot vectorize a fragment")]
    NonRootNode,
    #[error("failed to write to the output stream")]
    Write(#[from] std::fmt::Error),
    #[error("failed to vectorize text")]
    TextVectorize(#[from] TextVectorizeError),
}

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
    pub(crate) layout: taffy::Style,
    pub(crate) visual: Appearance,
    pub(crate) children: Vec<usize>,
    pub(crate) resources: Vec<Resource>,
    pub(crate) typography: Typography,
    // computed
    pub(crate) cache: taffy::Cache,
    pub(crate) unrounded_layout: taffy::Layout,
    pub(crate) final_layout: taffy::Layout,
    pub(crate) scaled_radii: ScaledRadii,
}

impl Node {
    pub(crate) fn new(
        kind: NodeKind,
        layout: taffy::Style,
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
            cache: taffy::Cache::new(),
            unrounded_layout: taffy::Layout::with_order(0),
            final_layout: taffy::Layout::with_order(0),
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

    fn has_border(&self) -> bool {
        let taffy::Rect {
            top,
            right,
            bottom,
            left,
        } = self.final_layout.border;
        !self.visual.border.is_none() && (top + right + bottom + left) > 0.0
    }

    fn has_radius(&self) -> bool {
        let radius = self.visual.corner_radius;
        !radius.top_left.is_zero()
            || !radius.top_right.is_zero()
            || !radius.bottom_left.is_zero()
            || !radius.bottom_right.is_zero()
    }

    fn should_clip(&self) -> (bool, bool) {
        let clip_x = self.layout.overflow.x == taffy::Overflow::Hidden;
        let clip_y = self.layout.overflow.y == taffy::Overflow::Hidden;
        (clip_x, clip_y)
    }

    fn open_block_group<T>(&self, ctx: &mut RenderContext<T>) -> Result<(), VectorizeError>
    where
        T: Write,
    {
        ElementWriter::new(ctx.out, "g")?
            .attr_if("opacity", self.visual.opacity, self.visual.opacity != 1.0)?
            .attr_if(
                "filter",
                (format_args!("url(#{})", self.visual.filter.iri()),),
                !self.visual.filter.is_default(),
            )?
            .attr_if(
                "style",
                (format_args!("mix-blend-mode:{}", self.visual.blend_mode),),
                !self.visual.blend_mode.is_default(),
            )?
            .write(|out| {
                self.visual.transform.write(
                    out,
                    (0.0, 0.0),
                    (self.final_layout.location.x, self.final_layout.location.y),
                    (self.final_layout.size.width, self.final_layout.size.height),
                )
            })?
            .open()
            .map(|_| ())
            .map_err(Into::into)
    }

    fn render_block_background<T>(&self, ctx: &mut RenderContext<T>) -> Result<(), VectorizeError>
    where
        T: Write,
    {
        if self.visual.background.is_none() {
            return Ok(());
        }

        let taffy::Size {
            width: w,
            height: h,
        } = self.final_layout.size;
        let radius = self.scaled_radii;
        let layers = self.visual.background.layers();

        if layers.len() == 1 {
            ElementWriter::new(ctx.out, "path")?
                .write_attr("d", |out| write_fill_path(out, w, h, radius))?
                .attr("fill", (&layers[0].paint,))?
                .attr_if(
                    "fill-opacity",
                    layers[0].opacity,
                    layers[0].opacity != NormalizedF32::ONE,
                )?
                .attr_if(
                    "style",
                    (format_args!("mix-blend-mode:{}", layers[0].blend_mode),),
                    !layers[0].blend_mode.is_default(),
                )?
                .close()
        } else {
            let path = Path::build(|buf| write_fill_path(buf, w, h, radius))?;
            let path_href = format_args!("#{}", path.iri());
            ctx.resources.lock().get_or_add_resource(path.into());

            ElementWriter::new(ctx.out, "g")?
                .attr_if(
                    "style",
                    "isolation:isolate",
                    self.visual.background.needs_isolation(),
                )?
                .content(|out| {
                    layers.iter().try_for_each(|layer| {
                        ElementWriter::new(out, "use")?
                            .attr("href", (path_href,))?
                            .attr("fill", (&layer.paint,))?
                            .attr_if(
                                "fill-opacity",
                                layer.opacity,
                                layer.opacity != NormalizedF32::ONE,
                            )?
                            .attr_if(
                                "style",
                                (&format_args!("mix-blend-mode:{}", layer.blend_mode),),
                                !layer.blend_mode.is_default(),
                            )?
                            .close()
                    })
                })?
                .close()
        }
        .map_err(Into::into)
    }

    fn render_block_border<T>(&self, ctx: &mut RenderContext<T>) -> Result<(), VectorizeError>
    where
        T: Write,
    {
        if !self.has_border() {
            return Ok(());
        }

        let taffy::Size {
            width: w,
            height: h,
        } = self.final_layout.size;
        let border = Rect::from(self.final_layout.border);
        let layers = self.visual.border.layers();

        if layers.len() == 1 {
            ElementWriter::new(ctx.out, "path")?
                .write_attr("d", |out| {
                    write_border_path(out, w, h, self.scaled_radii, border)
                })?
                .attr("fill", (&layers[0].paint,))?
                .attr_if(
                    "fill-opacity",
                    layers[0].opacity,
                    layers[0].opacity != NormalizedF32::ONE,
                )?
                .attr_if(
                    "style",
                    (format_args!("mix-blend-mode:{}", layers[0].blend_mode),),
                    !layers[0].blend_mode.is_default(),
                )?
                .attrs([("fill-rule", "evenodd"), ("clip-rule", "evenodd")])?
                .close()
        } else {
            let path = Path::build(|buf| write_border_path(buf, w, h, self.scaled_radii, border))?;
            let path_href = format_args!("#{}", path.iri());
            ctx.resources.lock().get_or_add_resource(path.into());

            ElementWriter::new(ctx.out, "g")?
                .attr_if(
                    "style",
                    "isolation:isolate",
                    self.visual.border.needs_isolation(),
                )?
                .content(|out| {
                    layers.iter().try_for_each(|layer| {
                        ElementWriter::new(out, "use")?
                            .attr("href", (path_href,))?
                            .attr("fill", (&layer.paint,))?
                            .attrs([("fill-rule", "evenodd"), ("clip-rule", "evenodd")])?
                            .attr_if(
                                "fill-opacity",
                                layer.opacity,
                                layer.opacity != NormalizedF32::ONE,
                            )?
                            .attr_if(
                                "style",
                                (&format_args!("mix-blend-mode:{}", layer.blend_mode),),
                                !layer.blend_mode.is_default(),
                            )?
                            .close()
                    })
                })?
                .close()
        }
        .map_err(Into::into)
    }

    fn open_block_clip<T>(
        &self,
        ctx: &mut RenderContext<T>,
        (clip_x, clip_y): (bool, bool),
    ) -> Result<(), VectorizeError>
    where
        T: Write,
    {
        if !clip_x && !clip_y {
            return Ok(());
        }

        let taffy::Size {
            width: w,
            height: h,
        } = self.final_layout.size;
        let radius = self.scaled_radii;
        let border = Rect::from(self.final_layout.border);
        let clip = ClipPath::build(|out| {
            ElementWriter::new(out, "path")?
                .write_attr("d", |out| {
                    write_clip_path(out, w, h, radius, border, clip_x, clip_y, ctx.root_size)
                })?
                .close()
        })?;

        ElementWriter::new(ctx.out, "g")?
            .attr("clip-path", (format_args!("url(#{})", clip.iri()),))?
            .open()?;

        ctx.resources.lock().get_or_add_resource(clip.into());

        Ok(())
    }

    fn close_block_group<T>(clipped: bool, ctx: &mut RenderContext<T>) -> Result<(), VectorizeError>
    where
        T: Write,
    {
        if clipped {
            ElementWriter::close_tag(ctx.out, "g")?;
        }

        ElementWriter::close_tag(ctx.out, "g").map_err(Into::into)
    }

    pub(crate) fn pre_emit<T>(&self, ctx: &mut RenderContext<T>) -> Result<(), VectorizeError>
    where
        T: Write,
    {
        match &self.kind {
            NodeKind::Root(meta) => {
                let (w, h) = (meta.width, meta.height);
                let mut svg = ElementWriter::new(ctx.out, "svg")?
                    .attr_if(
                        "xmlns",
                        "http://www.w3.org/2000/svg",
                        !ctx.options.omit_svg_xmlns,
                    )?
                    .attr("viewBox", (format_args!("0 0 {w} {h}"),))?;

                match &ctx.options.svg_dimensions {
                    SvgDimensions::Omit => {}
                    SvgDimensions::Layout => {
                        svg = svg.attrs([("width", w), ("height", h)])?;
                    }
                    SvgDimensions::Custom { width, height } => {
                        svg =
                            svg.attrs([("width", width.as_str()), ("height", height.as_str())])?;
                    }
                };

                svg.open()?;
            }
            //
            NodeKind::Block
            | NodeKind::Flex
            | NodeKind::Column
            | NodeKind::Row
            | NodeKind::Grid => {
                self.open_block_group(ctx)?;
                self.render_block_background(ctx)?;
                self.render_block_border(ctx)?;
                self.open_block_clip(ctx, self.should_clip())?;
            }
            //
            NodeKind::Text(meta) => {
                let mut fonts = ctx.fonts.lock();
                let FontRegistry {
                    swash_cache,
                    system,
                    ..
                } = &mut *fonts;

                meta.vectorize_text(
                    ctx.out,
                    (self.final_layout.location.x, self.final_layout.location.y),
                    &self.visual,
                    swash_cache,
                    system,
                )?;
            }
            //
            NodeKind::Image(meta) => {
                let has_radius = self.has_radius();

                self.open_block_group(ctx)?;
                self.render_block_background(ctx)?;
                self.render_block_border(ctx)?;
                self.open_block_clip(ctx, (has_radius, has_radius))?;

                match &meta.source {
                    ImageSource::Url(_) | ImageSource::DataUri(_) => {
                        ElementWriter::new(ctx.out, "image")?
                            .attr("href", (&meta.source,))?
                            .attrs([
                                ("width", self.final_layout.size.width),
                                ("height", self.final_layout.size.height),
                            ])?
                            .attr("crossorigin", meta.cross_origin.map(|x| (x,)))?
                            .close()?;
                    }
                    ImageSource::Svg(svg) => {
                        ctx.out.write_str(svg)?;
                    }
                };

                Self::close_block_group(has_radius, ctx)?;
            }
        };

        Ok(())
    }

    pub(crate) fn post_emit<T>(&self, ctx: &mut RenderContext<T>) -> Result<(), VectorizeError>
    where
        T: Write,
    {
        match &self.kind {
            NodeKind::Root(_) => {
                let resources = ctx.resources.lock();

                if !resources.is_empty() {
                    ElementWriter::new(ctx.out, "defs")?
                        .content(|out| out.write_fmt(format_args!("{resources}")))?
                        .close()?;
                }

                ElementWriter::close_tag(ctx.out, "svg")?;
            }
            //
            NodeKind::Block
            | NodeKind::Flex
            | NodeKind::Column
            | NodeKind::Row
            | NodeKind::Grid => {
                Self::close_block_group(
                    self.layout.overflow.x == taffy::Overflow::Hidden
                        || self.layout.overflow.y == taffy::Overflow::Hidden,
                    ctx,
                )?;
            }
            //
            NodeKind::Text(_) | NodeKind::Image(_) => {}
        }

        Ok(())
    }
}
