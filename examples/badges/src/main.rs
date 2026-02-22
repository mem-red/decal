use decal::prelude::*;
use std::fs;

const GITHUB_ICON: &'static str = r##"<svg viewBox="0 0 16 16" width="22" height="22" fill="#8492af"><path d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z"></path></svg>"##;

fn make_badge(label: &str, value: &str, color: u32, icon: Option<&str>) -> Scene {
    decal! {
        Row {
            Row {
                if let Some(svg) = icon {
                    Image(ImageSource::svg(svg), 22.0, 22.0)
                }

                Text(label)
            }
                .padding((10, 14))
                .gap(14)
                .align_items(AlignItems::Center)
                .background(rgb(0x2d2d3a))

            Row {
                Text(value).width(pct(100.0))
            }
                .flex_grow(1.0)
                .padding((10, 14))
                .text_align(TextAlign::Center)
        }
            .overflow(Overflow::Hidden)
            .corner_radius(8.0)
            .background(rgb(color))
    }
}

fn main() {
    let mut engine = Engine::new(EngineOptions::default());
    let mut badges = decal! {
        Column {
            Scene(make_badge("build", "passing", 0x2a7a40, Some(GITHUB_ICON)))
            Scene(make_badge("version", "0.5.0", 0x2a4a7a, None))
            Scene(make_badge("license", "MIT", 0x4a2a7a, None))
        }
            .gap(16)
            .padding(32)
            .font_size(22.0)
            .line_height(28.0)
            .font_family("monospace")
            .font_weight(FontWeight::Bold)
            .color(rgb(0xd8d8d8))
            .background(rgb(0x0))
    };

    let (svg, _size) = engine
        .vectorize(&mut badges, &VectorizeOptions::default())
        .unwrap();

    fs::write("./output.svg", svg).unwrap();

    let (pixmap, _size) = engine
        .rasterize(&mut badges, &RasterizeOptions::default())
        .unwrap();

    pixmap.save_png("./output.png").unwrap();
}
