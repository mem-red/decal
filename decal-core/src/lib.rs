//! # Decal
//!
//! `decal` is a declarative rendering library that lets you describe scenes
//! using a Rust-native DSL and render them to SVG or PNG.
//!
//! Scene descriptions are backend-agnostic and can be serialized into vector
//! (SVG, native) or raster (PNG using [`resvg`]) output while preserving
//! layout semantics and visual fidelity.
//!
//! ## Features
//!
//! - Declarative node builders for flexbox, text, images, containers, etc.
//! - Layout capabilities modeled after CSS box concepts (gap, alignment,
//!   flex/grid).
//! - Support for complex paint stacks, gradients, text stencils, and SVG
//!   filters.
//!
//! ## Examples
//!
//! ```rust
//! use decal::prelude::*;
//! use std::fs;
//!
//! let mut engine = Engine::new(EngineOptions::default());
//!
//! let mut scene = decal! {
//!     Column {
//!         Text("hello")
//!             .color(rgb(0xffffff))
//!     }
//!         .padding(32.0)
//!         .background(LinearGradient::right().stops([(0.0, rgb(0xff0000)), (1.0, rgb(0x00ff00))]))
//! };
//!
//! let (svg, _scene_size) = engine
//!     .vectorize(&mut scene, &VectorizeOptions::default())
//!     .unwrap();
//!
//! fs::write("./markup.svg", svg).unwrap();
//!
//! let (pixmap, _scene_size) = engine
//!     .rasterize(&mut scene, &RasterizeOptions::default())
//!     .unwrap();
//!
//! pixmap.save_png("./render.png").unwrap();
//! ```
//!
//! Checkout the project repository for more details: https://github.com/mem-red/decal

#[forbid(unsafe_code)]
#[warn(missing_docs)]
//
// Private
mod macros;
mod paint;
#[cfg(test)]
mod test_utils;
mod utils;
//
// Public
pub mod attributes;
pub mod builders;
pub mod capabilities;
pub mod filters;
pub mod layout;
pub mod prelude;
pub mod primitives;
//
// Public macros re-export
pub use decal_macros::*;
