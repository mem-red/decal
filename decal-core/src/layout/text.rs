use crate::builders::TextSpan;
use crate::layout::{BASE_FONT_SIZE, BASE_LINE_HEIGHT, FontRegistry};
use crate::layout::{DEFAULT_FONT_FAMILY, Typography};
use crate::paint::Appearance;
use crate::prelude::Color;
use crate::text::{FontStyle, FontWeight};
use crate::utils::{PathWriter, encode_image};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use cosmic_text::{Attrs, Buffer, Command, Family, FontSystem, Metrics, Shaping, SwashCache};
use png::EncodingError;
use std::fmt::Formatter;
use std::sync::{Arc, Mutex};
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

#[derive(Debug, Clone, Default)]
pub(crate) struct TextMeta {
    spans: Vec<TextSpan>,
    buffer: Option<Buffer>,
    typography: Typography,
    width: f32,
    height: f32,
}

impl TextMeta {
    pub(crate) fn new(spans: Vec<TextSpan>) -> Self {
        Self {
            spans,
            buffer: None,
            typography: Typography::default(),
            width: 0.0,
            height: 0.0,
        }
    }

    pub(crate) fn set_typography(&mut self, typography: Typography) {
        self.typography = typography;
    }

    pub(crate) fn vectorize_text<T>(
        &self,
        out: &mut T,
        offset: (f32, f32),
        appearance: &Appearance,
        cache: &mut SwashCache,
        font_system: &mut FontSystem,
    ) -> Result<(), TextVectorizeError>
    where
        T: std::fmt::Write,
    {
        let ref transform = appearance.transform;
        let Some(ref buffer) = self.buffer else {
            return Ok(());
        };

        write!(out, r#"<g"#)?;

        if appearance.opacity != 1.0 {
            write!(out, r#" opacity="{}" "#, appearance.opacity)?;
        }

        transform.write(out, offset, (0.0, 0.0), (self.width, self.height))?;
        write!(out, r#" >"#)?;

        for run in buffer.layout_runs() {
            let line_y = run.line_y;

            for glyph in run.glyphs.iter() {
                let physical = glyph.physical(offset, 1.0);
                let glyph_x = physical.x as f32;
                let glyph_y = physical.y as f32;
                let cache_key = physical.cache_key;

                if let Some(outline_commands) = cache
                    .get_outline_commands(font_system, cache_key)
                    .filter(|x| is_drawable(*x))
                {
                    let span_fill = self
                        .spans
                        .get(glyph.metadata)
                        .and_then(|span| span.typography.color.clone())
                        .unwrap_or(DEFAULT_COLOR.into());

                    write!(out, r#"<path fill="{span_fill}" d=""#)?;

                    let mut d = PathWriter::new(out);

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
                } else if let Some(image) = cache.get_image(font_system, cache_key) {
                    // handle emoji/color glyphs
                    if image.content == Content::Color {
                        let x = glyph_x + image.placement.left as f32;
                        let y = line_y + glyph_y - image.placement.top as f32;
                        let w = image.placement.width as f32;
                        let h = image.placement.height as f32;

                        write!(
                            out,
                            r#"<image href="data:image/png;base64,{}" x="{x}" y="{y}" width="{w}" height="{h}" />"#,
                            BASE64.encode(encode_image(&image)?),
                        )?;
                    }
                }
            }
        }

        write!(out, r#"</g>"#)?;

        Ok(())
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

        let Ok(mut fonts) = fonts.lock() else {
            return Size::zero();
        };

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

        self.width = width;
        self.height = height;

        Size { width, height }
    }

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
            brw.set_wrap(wrap.to_cosmic_wrap());
        }

        brw.set_rich_text(
            spans,
            &root_attrs,
            Shaping::Advanced,
            self.typography.align.map(|x| x.to_cosmic_align()),
        );

        self.buffer = Some(brw.to_owned());
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
        .style(tp.style.unwrap_or(FontStyle::Normal).to_cosmic_style())
        .weight(tp.weight.unwrap_or(FontWeight::Normal).to_cosmic_weight());

    if let Some(letter_spacing) = tp.letter_spacing {
        attrs = attrs.letter_spacing(letter_spacing);
    }

    (attrs, metrics)
}
