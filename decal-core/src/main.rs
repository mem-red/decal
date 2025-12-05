use decal::prelude::*;
use decal_macros::{decal, fragment};
use std::time::Instant;

fn heading(owner: &str, repo: &str) -> Decal {
    fragment! {
        Row {
            Text(owner)
            Text("/")
            Text(repo)
                .font_weight(FontWeight::Black)
        }
            .font_size(64)
            .line_height(96)
    }
}

fn stat(value: usize, label: &str) -> Decal {
    fragment! {
        Column {
            Text(value.to_string())
            Text(label)
        }
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
                        Text("RepoImg")
                    }

                    // Stats
                    Row {
                        Fragment(stat(82, "Contributors"))
                    }

                    Row {
                        Block {}.background([243, 75, 125]).width(Length::percent(61.2))
                        Block {}.background([227, 76, 37]).width(Length::percent(17.5))
                        Block {}.background([53, 114, 165]).width(Length::percent(8.9))
                        Block {}.background([61, 97, 24]).width(Length::percent(5.1))
                        Block {}.background([176, 114, 25]).width(Length::percent(3.2))
                        Block {}.background([236, 222, 190]).width(Length::percent(2.1))
                        Block {}.background([85, 85, 85]).width(Length::percent(1.3))
                        Block {}.background([240, 224, 90]).width(Length::percent(0.7))
                    }
                        .size([Length::percent(100), Length::pixels(32)])
                        .margin_top(Length::auto())
                }
                    .background([255, 255, 255])
                    .size([Length::percent(100), Length::percent(100)])
            }
                .font_size(32)
                .line_height(46)
                .color([48, 54, 62])
                .font_family("MonaSans")
    };

    let mut engine = Engine::new(EngineOptions {
        fonts: FontRegistry::new().load_font("MonaSans", include_bytes!("../MonaSans.ttf")),
    });
    let start = Instant::now();
    let pixmap = engine.rasterize(&mut dcl, None, None).unwrap();
    println!("rasterize: {:.3} ms", start.elapsed().as_millis());
    pixmap.save_png("./output.png").unwrap();
    println!("save to disk: {:.3} ms", start.elapsed().as_millis());
}
