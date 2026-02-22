use decal::prelude::*;
use std::fs;

fn main() {
    let mut engine = Engine::new(EngineOptions::default());

    let mut surface = decal! {
        Block {}
            .size((640, 320))
            .background(rgb(0x0))
            .fx(
                Filter::new(|ctx| {
                    ctx.composite()
                        .input(
                            ctx.specular_lighting(LightSource::distant_light(225.0, 35.0))
                                .input(
                                    ctx.turbulence()
                                        .fractal_noise()
                                        .base_freq(0.1)
                                        .num_octaves(4)
                                        .finish(),
                                )
                                .surface_scale(5.0)
                                .specular_exponent(20.0)
                                .lighting_color(rgb((255, 240, 200)))
                                .finish(),
                        )
                        .input2(FilterInput::source_alpha())
                        .operator(CompositeOperator::r#in())
                        .finish();
                })
            )
    };

    let (svg, _size) = engine
        .vectorize(&mut surface, &VectorizeOptions::default())
        .unwrap();

    fs::write("./output.svg", svg).unwrap();

    let (pixmap, _size) = engine
        .rasterize(&mut surface, &RasterizeOptions::default())
        .unwrap();

    pixmap.save_png("./output.png").unwrap();
}
