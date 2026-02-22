use decal::prelude::*;
use std::fs;
use taffy::prelude::*;

// Reference: https://dev.to/madsstoumann/the-golden-ratio-in-css-53d0

fn main() {
    let mut engine = Engine::new(EngineOptions::default());
    let children = [
        (0xe47a2c, (1, 22), (1, 22)),
        (0xbaccc0, (1, 23), (22, 35)),
        (0x6c958f, (14, 22), (27, 35)),
        (0x40363f, (17, 22), (22, 27)),
        (0xd7a26c, (14, 17), (22, 25)),
        (0xae4935, (14, 17), (25, 27)),
        (0xf7e6d4, (16, 17), (26, 27)),
        (0x2f3e46, (16, 17), (25, 26)),
    ];

    let mut golden_ratio = decal! {
        Grid {
            for (color, r, c) in children {
                Block {}
                    .bg(rgb(color))
                    .grid_row(Line { start: line(r.0), end: line(r.1) })
                    .grid_column(Line { start: line(c.0), end: line(c.1) })
            }
        }
            .grid_template_columns(vec![fr(1.0); 34])
            .grid_template_rows(vec![fr(1.0); 21])
            .size((800.0, 800.0 * 21.0 / 34.0))
    };

    let (svg, _size) = engine
        .vectorize(&mut golden_ratio, &VectorizeOptions::default())
        .unwrap();

    fs::write("./output.svg", svg).unwrap();

    let (pixmap, _size) = engine
        .rasterize(&mut golden_ratio, &RasterizeOptions::default())
        .unwrap();

    pixmap.save_png("./output.png").unwrap();
}
