use decal::prelude::*;
use std::f32::consts::FRAC_1_SQRT_2;
use std::fs;

fn main() {
    let mut engine = Engine::new(EngineOptions::default());

    let gradient_list = [
        LinearGradient::angle(336.0)
            .stops([(0.0, rgb(0x0000ff)), (FRAC_1_SQRT_2, rgba(0x0000ff00))]),
        LinearGradient::angle(127.0)
            .stops([(0.0, rgb(0x00ff00)), (FRAC_1_SQRT_2, rgba(0x00ff0000))]),
        LinearGradient::angle(217.0)
            .stops([(0.0, rgb(0xff0000)), (FRAC_1_SQRT_2, rgba(0xff000000))]),
    ];

    let mut gradients = decal! {
        Root(640.0, 480.0) {
            Block {}
                .size(pct(100.0))
                .background(gradient_list)
        }
    };

    let (svg, _size) = engine
        .vectorize(&mut gradients, &VectorizeOptions::default())
        .unwrap();

    fs::write("./output.svg", svg).unwrap();

    let (pixmap, _size) = engine
        .rasterize(&mut gradients, &RasterizeOptions::default())
        .unwrap();

    pixmap.save_png("./output.png").unwrap();
}
