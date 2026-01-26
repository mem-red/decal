# Decal

[![Latest Version](https://img.shields.io/crates/v/decal.svg)](https://crates.io/crates/decal)
[![Rust Documentation](https://docs.rs/decal/badge.svg)](https://docs.rs/decal)

A declarative that lets you describe scene using a Rust DSL, then render it to SVG or PNG.

## Usage

At a high level, the flow looks like this:

1. Build a scene using the `decal!` macro and primitives like `Row`, `Column`, `Text`, `Image`, etc.
2. Initialize an `Engine` with fonts and options.
3. Render the scene to SVG (`vectorize`) or rasterize it to a bitmap (`rasterize`).

```rust
use decal::prelude::*;
use std::fs;

fn main() {
    let mut engine = Engine::new(EngineOptions::default());

    let mut scene = decal! {
        Block {}
            .size(256)
            .background(rgb(0xffffff))
    };

    let (svg, _size) = engine
        .vectorize(&mut scene, &VectorizeOptions::default())
        .unwrap();

    fs::write("./markup.svg", svg).unwrap();

    let (pixmap, _size) = engine
        .rasterize(&mut scene, &RasterizeOptions::default())
        .unwrap();

    pixmap.save_png("./render.png").unwrap();
}
```

## Examples

### Basic layout

```rust
decal! {
  Row {
    Image(
      ImageSource::svg(r##"<svg xmlns="http://www.w3.org/2000/svg" width="256" height="256" viewBox="0 0 128 128"><path fill="#fff" d="M56.7937 84.9688C44.4187 83.4688 35.7 74.5625 35.7 63.0313c0-4.6875 1.6875-9.75 4.5-13.125-1.2188-3.0938-1.0313-9.6563.375-12.375 3.75-.4688 8.8125 1.5 11.8125 4.2187 3.5625-1.125 7.3125-1.6875 11.9062-1.6875s8.3438.5625 11.7188 1.5938c2.9062-2.625 8.0625-4.5938 11.8125-4.125 1.3125 2.5312 1.5 9.0937.2812 12.2812 3 3.5625 4.5938 8.3438 4.5938 13.2188 0 11.5312-8.7188 20.25-21.2813 21.8437 3.1875 2.0625 5.3438 6.5625 5.3438 11.7188v9.7502c0 2.812 2.3437 4.406 5.1562 3.281C98.8875 103.156 112.2 86.1875 112.2 65.1875 112.2 38.6563 90.6375 17 64.1062 17 37.575 17 16.2 38.6562 16.2 65.1875 16.2 86 29.4187 103.25 47.2312 109.719c2.5313.937 4.9688-.75 4.9688-3.281v-7.5005c-1.3125.5625-3 .9375-4.5.9375-6.1875 0-9.8438-3.375-12.4688-9.6562-1.0312-2.5313-2.1562-4.0313-4.3125-4.3125-1.125-.0938-1.5-.5625-1.5-1.125 0-1.125 1.875-1.9688 3.75-1.9688 2.7188 0 5.0625 1.6875 7.5 5.1563 1.875 2.7187 3.8438 3.9375 6.1875 3.9375s3.8438-.8438 6-3c1.5938-1.5938 2.8125-3 3.9375-3.9375"/></svg>"##),
      256.0,
      256.0
    )
      .background(rgb(0x0))
      .corner_radius(48.0)
    Column {
      Text("GitHub")
        .font_size(112.0)
        .line_height(124.0)
      Text("Where the world builds software")
        .opacity(0.65)
        .font_size(86.0)
        .line_height(92.0)
    }
  }
    .gap(48)
    .padding(42)
    .align_items(AlignItems::Center)
    .background(
      LinearGradient::bottom_left().stops([
        (0.0, rgb(0xf9ffdb)),
        (1.0, rgb(0xa6c6ff)),
      ])
    )
}
```

<img src="https://raw.githubusercontent.com/mem-red/decal/refs/heads/main/assets/example_1.png" style="width: 100%;"  alt="Rendered image" />

***

### Filters

```rust
decal! {
  Block {}
    .size((960, 480))
    .background(rgba((0, 0, 0, 1.0)))
    .fx(
        Filter::new(|ctx| {
          let noise = ctx
              .turbulence()
              .fractal_noise()
              .base_freq((0.05, 0.05))
              .num_octaves(5)
              .seed(1)
              .finish();

          let lighting = ctx
              .diffuse_lighting(LightSource::point_light(700.0, 180.0, 12.0))
              .input(noise)
              .surface_scale(0.3)
              .diffuse_constant(3.1)
              .lighting_color(rgb((255, 185, 255)))
              .finish();

          ctx.composite()
              .input(lighting)
              .input2(FilterInput::source_alpha())
              .operator(CompositeOperator::r#in())
              .finish();
        })
          .color_interpolation(ColorInterpolation::SRgb)
    )
}
```

<img src="https://raw.githubusercontent.com/mem-red/decal/refs/heads/main/assets/example_2.png" style="width: 100%;"  alt="Rendered image" />

***

<br />

Explore more [examples](https://github.com/mem-red/decal/tree/main/decal-core/examples).

## License

[MIT](https://github.com/mem-red/decal/blob/main/LICENSE-MIT)
or [Apache-2.0](https://github.com/mem-red/decal/blob/main/LICENSE-APACHE)
