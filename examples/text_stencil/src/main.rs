use decal::prelude::*;
use std::fs;

fn main() {
    let mut engine = Engine::new(EngineOptions {
        fonts: FontRegistry::new().load_system_fonts(),
        ..Default::default()
    });

    let gradient = LinearGradient::right().stops([
        (0.3, rgb(0x3f87a6)),
        (0.6, rgb(0xebf8e1)),
        (0.9, rgb(0xf69d3c)),
    ]);

    let mut stencil = decal! {
        Column {
            Text("type = alpha 🐠")
                .stencil(gradient.clone())
                .stencil_type(StencilType::Alpha)

            Text("type = luminance 🐠")
                .color(rgb(0xffffff))
                .stencil(gradient.clone())
                .stencil_type(StencilType::Luminance)

            Text("scope = vector glyphs 🐠")
                .stencil(gradient)
                .stencil_scope(StencilScope::VectorGlyphs)
        }
            .size(pct(100.0))
            .padding(48)
            .align_items(AlignItems::Stretch)
            .font_size(64.0)
            .line_height(80.0)
            .font_weight(FontWeight::Bold)
            .text_align(TextAlign::End)
            .background(rgb(0x000))
    };

    let (svg, _size) = engine
        .vectorize(&mut stencil, &VectorizeOptions::default())
        .unwrap();

    fs::write("./output.svg", svg).unwrap();

    let (pixmap, _size) = engine
        .rasterize(&mut stencil, &RasterizeOptions::default())
        .unwrap();

    pixmap.save_png("./output.png").unwrap();
}
