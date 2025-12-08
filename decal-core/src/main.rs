use decal::prelude::*;
use decal_macros::{decal, fragment, text};

fn heading(owner: &str, repo: &str) -> Decal {
    fragment! {
        Row {
            Text(text! {
                owner,
                "/",
                (repo, { weight: FontWeight::Bold })
             })
        }
            .font_size(64)
            .line_height(96)
    }
}

fn stat(value: usize, label: &str, icon: &str) -> Decal {
    fragment! {
        Row {
            Text(icon)
            Column {
                Text(value.to_string())
                    .color([0, 0, 0])
                Text(label)
                    .color([111, 119, 129])
                    .font_size(28)
                    .line_height(32)
            }
        }.gap(16.pix())
    }
}

fn main() {
    let mut dcl = decal! {
            Root(1200, 630) {
                Column {
                    Row {
                        Column {
                            Fragment(heading("nasa", "fprime"))
                            Text("F' - A flight software and embedded systems framework")
                                .color([110, 118, 129])
                        }
                            .width(70.pct())
                        Block {}
                            .size(196.pix())
                            .background([0, 0, 0])
                    }
                        .gap(64.pix())
                        .flex_grow(1.0)

                    // Stats
                    Row {
                        Fragment(stat(82, "Contributors", "C"))
                        Fragment(stat(53, "Issues", "I"))
                        Fragment(stat(6, "Discussions", "D"))
                        Fragment(stat(800, "Stars", "S"))
                        Fragment(stat(95, "Forks", "F"))
                    }
                        .justify_content(JustifyContent::SpaceBetween)

                    Row {
                        Block {}.background([243, 75, 125]).width(61.2.pct())
                        Block {}.background([227, 76, 37]).width(17.5.pct())
                        Block {}.background([53, 114, 165]).width(8.9.pct())
                        Block {}.background([61, 97, 24]).width(5.1.pct())
                        Block {}.background([176, 114, 25]).width(3.2.pct())
                        Block {}.background([236, 222, 190]).width(2.1.pct())
                        Block {}.background([85, 85, 85]).width(1.3.pct())
                        Block {}.background([240, 224, 90]).width(0.7.pct())
                    }
                        .height(28.pix())
                        .margin_x((-76).pix())
                        .margin_bottom((-76).pix())
                }
                    .padding(76.pix())
                    .align_items(AlignItems::Stretch)
                    .gap(48.pix())
                    .background([255, 255, 255])
                    .size(100.pct())
            }
                .font_size(32)
                .line_height(46)
                .color([48, 54, 62])
                .font_family("MonaSans")
                .text_wrap(TextWrap::Word)
    };

    let mut engine = Engine::new(EngineOptions {
        fonts: FontRegistry::new().load_font("MonaSans", include_bytes!("../MonaSans.ttf")),
    });
    let start = std::time::Instant::now();
    let pixmap = engine.rasterize(&mut dcl, None, None).unwrap();
    println!("rasterize: {:.3} ms", start.elapsed().as_millis());
    pixmap.save_png("./output.png").unwrap();
    println!("save to disk: {:.3} ms", start.elapsed().as_millis());
}
