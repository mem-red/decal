use crate::builders::TextSpan;
use crate::layout::{DEFAULT_FONT_FAMILY, Typography};
use crate::prelude::{BASE_FONT_SIZE, BASE_LINE_HEIGHT, FontRegistry};
use crate::text::{FontStyle, FontWeight};
use crate::utils::{PathBuilder, encode_image};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use cosmic_text::{Attrs, Buffer, Command, Family, FontSystem, Metrics, Shaping, SwashCache};
use png::EncodingError;
use std::sync::{Arc, Mutex};
use swash::scale::image::Content;
use taffy::prelude::*;
use thiserror::Error;
use zeno::Point;

const DEFAULT_COLOR: &'static str = "#000";

#[derive(Error, Debug)]
pub enum TextVectorizationError {
    #[error("cannot write to stream")]
    SvgWrite(#[from] std::fmt::Error),
    #[error("failed to encode image")]
    ImageEncoding(#[from] EncodingError),
}

#[derive(Debug, Clone)]
pub(crate) struct TextMeta {
    pub(crate) spans: Vec<TextSpan>,
    pub(crate) buffer: Buffer,
    pub(crate) typography: Typography,
}

impl TextMeta {
    pub(crate) fn new(spans: Vec<TextSpan>) -> Self {
        Self {
            spans,
            buffer: Buffer::new_empty(Metrics::new(BASE_FONT_SIZE, BASE_LINE_HEIGHT)),
            typography: Typography::default(),
        }
    }

    pub(crate) fn set_typography(&mut self, typography: Typography) {
        self.typography = typography;
    }

    pub(crate) fn write_vectorized_text<T>(
        &self,
        out: &mut T,
        offset: (f32, f32),
        font_system: &mut FontSystem,
    ) -> Result<(), TextVectorizationError>
    where
        T: std::fmt::Write,
    {
        let mut cache = SwashCache::new();

        for run in self.buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let physical = glyph.physical(offset, 1.0);
                let glyph_x = physical.x as f32;
                let glyph_y = physical.y as f32;
                let line_y = run.line_y;
                let cache_key = physical.cache_key;

                if let Some(image) = cache.get_image_uncached(font_system, cache_key) {
                    // handle emoji/color glyphs
                    if image.content == Content::Color {
                        write!(
                            out,
                            r#"<image href="data:image/png;base64,{}" x="{}" y="{}" width="{}" height="{}"/>"#,
                            BASE64.encode(encode_image(&image)?),
                            glyph_x + image.placement.left as f32,
                            line_y + glyph_y - image.placement.top as f32,
                            image.placement.width,
                            image.placement.height
                        )?;

                        continue;
                    }
                }

                if let Some(outline_commands) = cache.get_outline_commands(font_system, cache_key) {
                    if outline_commands.len() == 0 {
                        continue;
                    }

                    let span_fill = if let Some(span) = self.spans.get(glyph.metadata) {
                        span.typography.color
                    } else {
                        self.typography.color
                    }
                    .map_or(DEFAULT_COLOR.to_string(), |c| c.to_string());

                    write!(out, r#"<path fill="{span_fill}" d=""#)?;

                    let mut d = PathBuilder::new(out);

                    for command in outline_commands.iter() {
                        match *command {
                            Command::MoveTo(Point { x, y }) => {
                                d.move_to(x + glyph_x, line_y + glyph_y - y);
                            }
                            Command::LineTo(Point { x, y }) => {
                                d.line_to(x + glyph_x, line_y + glyph_y - y);
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
                                );
                            }
                            Command::QuadTo(Point { x: cx, y: cy }, Point { x, y }) => {
                                d.quad_to(
                                    cx + glyph_x,
                                    line_y + glyph_y - cy,
                                    x + glyph_x,
                                    line_y + glyph_y - y,
                                );
                            }
                            Command::Close => d.close()?,
                        }
                    }

                    write!(out, r#"" />"#)?;
                }
            }
        }

        Ok(())
    }

    pub(crate) fn measure(
        &mut self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        fonts: Arc<Mutex<FontRegistry>>,
    ) -> Size<f32> {
        let Ok(mut fonts) = fonts.lock() else {
            return Size::zero();
        };

        self.init_buffer(&mut fonts);

        if let Size {
            width: Some(width),
            height: Some(height),
        } = known_dimensions
        {
            return Size { width, height };
        }

        let width_constraint = known_dimensions.width.or(match available_space.width {
            AvailableSpace::MinContent => Some(0.0),
            AvailableSpace::MaxContent => None,
            AvailableSpace::Definite(width) => Some(width),
        });

        self.buffer
            .set_size(&mut fonts.system, width_constraint, None);
        self.buffer.shape_until_scroll(&mut fonts.system, false);

        let (width, total_lines) = self
            .buffer
            .layout_runs()
            .fold((0.0, 0usize), |(width, total_lines), run| {
                (run.line_w.max(width), total_lines + 1)
            });
        let height = total_lines as f32 * self.buffer.metrics().line_height;

        Size { width, height }
    }

    fn init_buffer(&mut self, fonts: &mut FontRegistry) {
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
            brw.set_wrap(wrap.to_cosmic_wrap());
        }

        brw.set_rich_text(
            spans,
            &root_attrs,
            Shaping::Advanced,
            self.typography.align.map(|x| x.to_cosmic_align()),
        );

        self.buffer = brw.to_owned();
    }
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
        .style(tp.style.unwrap_or(FontStyle::Normal).to_cosmic_style())
        .weight(tp.weight.unwrap_or(FontWeight::Normal).to_cosmic_weight());

    if let Some(letter_spacing) = tp.letter_spacing {
        attrs = attrs.letter_spacing(letter_spacing);
    }

    (attrs, metrics)
}
