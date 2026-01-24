# Decal

[![Latest Version](https://img.shields.io/crates/v/decal.svg)](https://crates.io/crates/decal)
[![Rust Documentation](https://docs.rs/decal/badge.svg)](https://docs.rs/decal)

A declarative library for building and rendering vector graphics.

<table>

<tr>
    <th> Markup </th>
    <th> Render </th>
</tr>

<tr>
<td>

```rust
decal! {
  Root(None, None) {
    Row {
      Image("https://avatars.githubusercontent.com/u/9919?s=256", 256.0, 256.0)
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
}
```

</td>
<td>

![Rendered image](https://github.com/mem-red/decal/blob/main/assets/example.png)

</td>
</tr>

</table>

Explore more [examples](https://github.com/mem-red/decal/tree/main/decal-core/examples).

# License

[MIT](https://github.com/mem-red/decal/blob/main/LICENSE-MIT)
or [Apache-2.0](https://github.com/mem-red/decal/blob/main/LICENSE-APACHE)