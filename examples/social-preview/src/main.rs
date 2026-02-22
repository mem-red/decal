use decal::prelude::*;
use std::fs;

fn main() {
    let mut engine = Engine::new(EngineOptions::default());

    // https://css-tricks.com/creating-patterns-with-svg-filters/#aa-island-group
    let backdrop = Filter::new(|ctx| {
        ctx.turbulence()
            .fractal_noise()
            .base_freq(0.01)
            .num_octaves(5)
            .finish();

        let matrix = ctx
            .color_matrix()
            .matrix([
                [1.0, 0.0, 0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0, 1.0],
            ])
            .finish();

        ctx.composite()
            .input(ctx.flood().color(rgb((0, 0, 0))).opacity(0.8).finish())
            .input2(
                ctx.component_transfer()
                    .input(matrix)
                    .func_r(TransferFunction::table(vec![
                        0.0, 0.02, 0.03, 0.03, 0.09, 0.12, 0.27, 0.91, 0.3, 0.03, 0.0, 0.0,
                    ]))
                    .func_g(TransferFunction::table(vec![
                        0.01, 0.09, 0.16, 0.18, 0.38, 0.48, 0.54, 0.73, 0.33, 0.09, 0.01, 0.01,
                    ]))
                    .func_b(TransferFunction::table(vec![
                        0.03, 0.17, 0.3, 0.25, 0.37, 0.42, 0.42, 0.6, 0.17, 0.01, 0.0, 0.0,
                    ]))
                    .finish(),
            )
            .operator(CompositeOperator::over())
            .finish();
    });

    let mut preview = decal! {
        Column {
            // background
            Block {}
                .size(pct(100.0))
                .margin_top(-80.0)
                .margin_left(-80.0)
                .position(Position::Absolute)
                .bg(rgb(0x0))
                .fx(backdrop)

            Text("Article")
                .font_size(28.0)
                .line_height(36.0)
                .font_weight(FontWeight::Bold)
                .font_family("monospace")
                .opacity(0.65)

            Text("Building a real-time collaborative editor in Rust")
                .font_size(72.0)
                .line_height(84.0)

            Block {}.height(40) // spacer

            Row {
                Image("https://avatars.githubusercontent.com/u/218499289?s=200", 64.0, 64.0)
                    .corner_radius(8.0)
                Column {
                    Text("MemRED")
                        .font_size(28.0)
                        .line_height(34.0)
                    Text("Feb 23, 2026 · 12 min read")
                        .font_size(24.0)
                        .line_height(30.0)
                        .opacity(0.65)
                }
            }
            .gap(16)
            .align_items(AlignItems::Center)
        }
        .size((1200, 630))
        .padding(80)
        .color(rgb(0xffffff))
        .justify_content(JustifyContent::SpaceBetween)
        .background(
            LinearGradient::top().stops([
                (0.0, rgb(0x6c5ce7)),
                (1.0, rgb(0xa29bfe)),
            ])
        )
    };

    let (svg, _size) = engine
        .vectorize(&mut preview, &VectorizeOptions::default())
        .unwrap();

    fs::write("./output.svg", svg).unwrap();

    let (pixmap, _size) = engine
        .rasterize(&mut preview, &RasterizeOptions::default())
        .unwrap();

    pixmap.save_png("./output.png").unwrap();
}
