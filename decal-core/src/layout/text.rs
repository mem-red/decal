use cosmic_text::{Attrs, Buffer, Command, FontSystem, Metrics, Shaping, SwashCache};
use taffy::prelude::*;
use zeno::Point;

#[derive(Debug, Clone)]
pub(crate) struct CosmicTextContext {
    buffer: cosmic_text::Buffer,
    metrics: Metrics,
}

impl CosmicTextContext {
    pub(crate) fn new(
        metrics: Metrics,
        spans: &[(&str, Attrs)],
        attrs: Attrs,
        font_system: &mut FontSystem,
    ) -> Self {
        let mut buffer = Buffer::new_empty(metrics);
        buffer.set_size(font_system, None, None);
        buffer.set_rich_text(
            font_system,
            spans.iter().map(|(text, attrs)| (*text, attrs.clone())),
            &attrs,
            Shaping::Advanced,
            None,
        );
        Self { buffer, metrics }
    }

    pub(crate) fn write_vertorized_text(&self, offset: (f32, f32), out: &mut String) {
        let mut font_system = FontSystem::new(); // TODO!
        let inter_variable = include_bytes!("../inter.ttf");
        font_system.db_mut().load_font_data(inter_variable.to_vec());

        let mut cache = SwashCache::new();
        let line_height = self.metrics.line_height;

        for run in self.buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let physical = glyph.physical(offset, 1.0);
                let glyph_x = physical.x as f32;
                let glyph_y = physical.y as f32;
                let cache_key = physical.cache_key;

                //
                //

                dbg!(run.text);
                dbg!(&physical.cache_key);

                let Some(image) = cache.get_image_uncached(&mut font_system, cache_key) else {
                    println!("No rasterization needed?");
                    continue;
                };

                dbg!(image.source);
                dbg!(image.content);
                dbg!(image.placement);
                dbg!(image.data.len());

                //
                //

                if let Some(outline_commands) =
                    cache.get_outline_commands(&mut font_system, cache_key)
                {
                    out.push_str(r#"<path d=""#);

                    for command in outline_commands.iter() {
                        match *command {
                            Command::MoveTo(Point { x, y }) => {
                                out.push_str(&format!(
                                    "M{} {} ",
                                    x + glyph_x,
                                    line_height - y + glyph_y
                                ));
                            }
                            Command::LineTo(Point { x, y }) => {
                                out.push_str(&format!(
                                    "L{} {} ",
                                    x + glyph_x,
                                    line_height - y + glyph_y
                                ));
                            }
                            Command::CurveTo(
                                Point { x: x1, y: y1 },
                                Point { x: x2, y: y2 },
                                Point { x, y },
                            ) => {
                                out.push_str(&format!(
                                    "C{} {} {} {} {} {} ",
                                    x1 + glyph_x,
                                    line_height - y1 + glyph_y,
                                    x2 + glyph_x,
                                    line_height - y2 + glyph_y,
                                    x + glyph_x,
                                    line_height - y + glyph_y
                                ));
                            }
                            Command::QuadTo(Point { x: cx, y: cy }, Point { x, y }) => {
                                out.push_str(&format!(
                                    "Q{} {} {} {} ",
                                    cx + glyph_x,
                                    line_height - cy + glyph_y,
                                    x + glyph_x,
                                    line_height - y + glyph_y
                                ));
                            }
                            Command::Close => out.push_str("Z"),
                        }
                    }

                    out.push_str(r#"" />"#);
                }
            }
        }
    }

    fn measure(
        &mut self,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
        font_system: &mut FontSystem,
    ) -> taffy::Size<f32> {
        // Set width constraint
        let width_constraint = known_dimensions.width.or(match available_space.width {
            AvailableSpace::MinContent => Some(0.0),
            AvailableSpace::MaxContent => None,
            AvailableSpace::Definite(width) => Some(width),
        });
        self.buffer.set_size(font_system, width_constraint, None);

        // Compute layout
        self.buffer.shape_until_scroll(font_system, false);

        // Determine measured size of text
        let (width, total_lines) = self
            .buffer
            .layout_runs()
            .fold((0.0, 0usize), |(width, total_lines), run| {
                (run.line_w.max(width), total_lines + 1)
            });
        let height = total_lines as f32 * self.buffer.metrics().line_height;

        taffy::Size { width, height }
    }
}

pub(crate) fn measure_function(
    known_dimensions: taffy::Size<Option<f32>>,
    available_space: taffy::Size<taffy::AvailableSpace>,
    text_context: Option<&mut CosmicTextContext>,
    font_system: &mut FontSystem,
) -> Size<f32> {
    if let Size {
        width: Some(width),
        height: Some(height),
    } = known_dimensions
    {
        return Size { width, height };
    }

    text_context.map_or(Size::ZERO, |ctx| {
        ctx.measure(known_dimensions, available_space, font_system)
    })
}
