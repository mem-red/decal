use crate::builders::TextSpan;
use crate::layout::{
    BASE_FONT_SIZE, BASE_LINE_HEIGHT, FontRegistry, RenderContext, Stencil, StencilScope,
    StencilType,
};
use crate::layout::{DEFAULT_FONT_FAMILY, Typography};
use crate::paint::{Iri, ResourceIri, ScaledRadii, write_fill_path};
use crate::primitives::Mask;
use crate::primitives::{Color, PaintStack};
use crate::text::{FontStyle, FontWeight};
use crate::utils::{ElementWriter, PathWriter, encode_image};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use cosmic_text::{Attrs, Buffer, Command, Family, Metrics, Shaping};
use parking_lot::Mutex;
use png::EncodingError;
use std::fmt::{Formatter, Write};
use std::sync::Arc;
use swash::scale::image::Content;
use taffy::prelude::*;
use thiserror::Error;
use zeno::Point;

const DEFAULT_COLOR: Color = Color::rgb(0, 0, 0);

#[derive(Error, Debug)]
pub enum TextVectorizeError {
    #[error("failed to write to the output stream")]
    Write(#[from] std::fmt::Error),
    #[error("failed to encode emoji image")]
    EncodeEmoji(#[from] EncodingError),
}

#[derive(Debug, Default)]
pub(crate) enum GlyphRenderMode {
    #[default]
    All,
    Vector,
    Bitmap,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct TextMeta {
    spans: Vec<TextSpan>,
    buffer: Option<Buffer>,
    typography: Typography,
    stencil: Stencil,
}

impl TextMeta {
    pub(crate) fn new(spans: Vec<TextSpan>) -> Self {
        Self {
            spans,
            ..Default::default()
        }
    }

    pub(crate) fn typography(&mut self, typography: Typography) {
        self.typography = typography;
    }

    pub(crate) fn stencil_paint(&mut self, value: PaintStack) {
        self.stencil.paint = value;
    }

    pub(crate) fn stencil_scope(&mut self, value: StencilScope) {
        self.stencil.scope = value;
    }

    pub(crate) fn stencil_type(&mut self, value: StencilType) {
        self.stencil.r#type = value;
    }

    pub(crate) fn measure(
        &mut self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        fonts: Arc<Mutex<FontRegistry>>,
    ) -> Size<f32> {
        if let Size {
            width: Some(width),
            height: Some(height),
        } = known_dimensions
        {
            return Size { width, height };
        }

        let mut fonts = fonts.lock();
        self.init_buffer(&mut fonts);

        let Some(ref mut buffer) = self.buffer else {
            return Size::zero();
        };

        let width_constraint = known_dimensions.width.or(match available_space.width {
            AvailableSpace::MinContent => Some(0.0),
            AvailableSpace::MaxContent => None,
            AvailableSpace::Definite(width) => Some(width),
        });

        buffer.set_size(&mut fonts.system, width_constraint, None);
        buffer.shape_until_scroll(&mut fonts.system, false);

        let (width, total_lines) = buffer
            .layout_runs()
            .fold((0.0, 0usize), |(width, total_lines), run| {
                (run.line_w.max(width), total_lines + 1)
            });
        let height = total_lines as f32 * buffer.metrics().line_height;

        Size { width, height }
    }

    pub(crate) fn render<W>(
        &self,
        ctx: &mut RenderContext<W>,
        layout: Layout,
    ) -> Result<(), TextVectorizeError>
    where
        W: Write,
    {
        if self.stencil.is_none() {
            self.render_text(ctx.out, &ctx.fonts, GlyphRenderMode::All)
        } else {
            let Size { width, height } = layout.size;
            let mask = {
                Mask::build(|out| {
                    self.render_text(
                        out,
                        &ctx.fonts,
                        if matches!(self.stencil.scope, StencilScope::VectorGlyphs) {
                            GlyphRenderMode::Vector
                        } else {
                            GlyphRenderMode::All
                        },
                    )
                    .map_err(|_| std::fmt::Error)
                })?
                .r#type(self.stencil.r#type.into())
            };

            self.render_stencil(ctx, mask.iri(), width, height)?;
            ctx.resources.lock().get_or_add_resource(mask.into());

            // render bitmaps on top
            if matches!(self.stencil.scope, StencilScope::VectorGlyphs) {
                self.render_text(ctx.out, &ctx.fonts, GlyphRenderMode::Bitmap)?;
            }

            Ok(())
        }
    }

    //

    fn init_buffer(&mut self, fonts: &mut FontRegistry) {
        if self.buffer.is_some() {
            return;
        }

        let mut spans = Vec::with_capacity(self.spans.len());

        for (idx, span) in self.spans.iter_mut().enumerate() {
            if span.hidden {
                continue;
            }

            span.typography.cascade_from(&self.typography);
            let (attrs, _) = typography_to_attrs(&mut span.typography, fonts);
            spans.push((span.content.as_str(), attrs.metadata(idx)));
        }

        let mut root_tp = self.typography.clone();
        let (root_attrs, root_metrics) = typography_to_attrs(&mut root_tp, fonts);
        let mut buf = Buffer::new_empty(root_metrics);
        let mut brw = buf.borrow_with(&mut fonts.system);

        if let Some(wrap) = self.typography.wrap {
            brw.set_wrap(wrap.into());
        }

        brw.set_rich_text(
            spans,
            &root_attrs,
            Shaping::Advanced,
            self.typography.align.map(Into::into),
        );

        self.buffer = Some(brw.to_owned());
    }

    fn render_text<W>(
        &self,
        out: &mut W,
        font_registry: &Arc<Mutex<FontRegistry>>,
        mode: GlyphRenderMode,
    ) -> Result<(), TextVectorizeError>
    where
        W: Write,
    {
        let Some(ref buffer) = self.buffer else {
            return Ok(());
        };

        let mut font_registry = font_registry.lock();
        let FontRegistry {
            swash_cache: cache,
            system: font_system,
            ..
        } = &mut *font_registry;

        let skip_vector = matches!(mode, GlyphRenderMode::Bitmap);
        let skip_bitmap = matches!(mode, GlyphRenderMode::Vector);

        for run in buffer.layout_runs() {
            let line_y = run.line_y;

            for glyph in run.glyphs.iter() {
                let physical = glyph.physical((0.0, 0.0), 1.0);
                let glyph_x = physical.x as f32;
                let glyph_y = physical.y as f32;
                let cache_key = physical.cache_key;

                if let Some(outline_commands) = cache
                    .get_outline_commands(font_system, cache_key)
                    .filter(|x| is_drawable(*x))
                {
                    if skip_vector {
                        continue;
                    }

                    ElementWriter::new(out, "path")?
                        .attr(
                            "fill",
                            (self
                                .spans
                                .get(glyph.metadata)
                                .and_then(|span| span.typography.color.clone())
                                .unwrap_or(DEFAULT_COLOR.into()),),
                        )?
                        .write_attr("d", |out| {
                            let mut d = PathWriter::new(out);

                            for command in outline_commands.iter() {
                                match *command {
                                    Command::MoveTo(Point { x, y }) => {
                                        d.move_to(x + glyph_x, line_y + glyph_y - y)?;
                                    }
                                    Command::LineTo(Point { x, y }) => {
                                        d.line_to(x + glyph_x, line_y + glyph_y - y)?;
                                    }
                                    Command::CurveTo(
                                        Point { x: x1, y: y1 },
                                        Point { x: x2, y: y2 },
                                        Point { x, y },
                                    ) => {
                                        d.curve_to(
                                            x1 + glyph_x,
                                            line_y + glyph_y - y1,
                                            x2 + glyph_x,
                                            line_y + glyph_y - y2,
                                            x + glyph_x,
                                            line_y + glyph_y - y,
                                        )?;
                                    }
                                    Command::QuadTo(Point { x: cx, y: cy }, Point { x, y }) => {
                                        d.quad_to(
                                            cx + glyph_x,
                                            line_y + glyph_y - cy,
                                            x + glyph_x,
                                            line_y + glyph_y - y,
                                        )?;
                                    }
                                    Command::Close => d.close()?,
                                }
                            }

                            Ok(())
                        })?
                        .close()?;
                } else if let Some(image) = cache.get_image(font_system, cache_key) {
                    // handle emoji/color glyphs
                    if !skip_bitmap && image.content == Content::Color {
                        ElementWriter::new(out, "image")?
                            .attr(
                                "href",
                                (format_args!(
                                    "data:image/png;base64,{}",
                                    BASE64.encode(encode_image(&image)?)
                                ),),
                            )?
                            .attrs([
                                ("x", glyph_x + image.placement.left as f32),
                                ("y", line_y + glyph_y - image.placement.top as f32),
                                ("width", image.placement.width as f32),
                                ("height", image.placement.height as f32),
                            ])?
                            .close()?;
                    }
                }
            }
        }

        Ok(())
    }

    fn render_stencil<W>(
        &self,
        ctx: &mut RenderContext<W>,
        mask_iri: Iri,
        width: f32,
        height: f32,
    ) -> std::fmt::Result
    where
        W: Write,
    {
        self.stencil.paint.render(
            ctx,
            |out| write_fill_path(out, width, height, ScaledRadii::default()),
            |out| write_fill_path(out, width, height, ScaledRadii::default()),
            |layer, is_use_element| {
                if is_use_element {
                    Ok(layer)
                } else {
                    layer.attr("mask", (format_args!("url(#{mask_iri})"),))
                }
            },
            |group| group.attr("mask", (format_args!("url(#{mask_iri})"),)),
        )
    }
}

impl std::fmt::Display for TextMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .spans
                .iter()
                .map(|x| x.content.clone())
                .collect::<Vec<_>>()
                .join(""),
        )
    }
}

fn is_drawable(cmds: &[Command]) -> bool {
    for cmd in cmds {
        match cmd {
            Command::LineTo(_) | Command::QuadTo(_, _) | Command::CurveTo(_, _, _) => return true,
            _ => {}
        }
    }

    false
}

fn typography_to_attrs<'a>(
    tp: &'a mut Typography,
    fonts: &mut FontRegistry,
) -> (Attrs<'a>, Metrics) {
    let metrics = Metrics {
        font_size: tp.size.unwrap_or(BASE_FONT_SIZE),
        line_height: tp.line_height.unwrap_or(BASE_LINE_HEIGHT),
    };

    let alias = tp.family.clone().unwrap_or(fonts.get_default_family());
    tp.resolved_family = fonts
        .resolve_family_name(alias.as_str())
        .unwrap_or(DEFAULT_FONT_FAMILY.to_string());

    let family = match tp.resolved_family.as_str() {
        "sans-serif" => Family::SansSerif,
        "serif" => Family::Serif,
        "mono" | "monospace" => Family::Monospace,
        name => Family::Name(name),
    };

    let mut attrs = Attrs::new()
        .family(family)
        .metrics(metrics)
        .style(tp.style.unwrap_or(FontStyle::Normal).into())
        .weight(tp.weight.unwrap_or(FontWeight::Normal).into());

    if let Some(letter_spacing) = tp.letter_spacing {
        attrs = attrs.letter_spacing(letter_spacing);
    }

    (attrs, metrics)
}
