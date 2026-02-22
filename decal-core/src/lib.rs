#![doc = include_str!("../../README.md")]

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
