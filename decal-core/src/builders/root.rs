use crate::layout::{Node, NodeKind};
use taffy::prelude::*;

#[derive(Debug)]
pub struct Root {
    meta: RootMeta,
    style: Style,
}

#[derive(Debug, Clone)]
pub(crate) struct RootMeta {
    width: f32,
    height: f32,
}

pub trait LengthExt {
    fn px(self) -> u32;
    fn pr(self) -> u32;
}

macro_rules! impl_length_ext_for_int {
    ($($t:ty),*) => {
        $(
            impl LengthExt for $t {
                fn px(self) -> u32 {
                    self as u32
                }
                fn pr(self) -> u32 {
                    self as u32
                }
            }
        )*
    }
}

impl_length_ext_for_int!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl Root {
    pub fn new<W, H>(width: W, height: H) -> Self
    where
        W: Into<f64>,
        H: Into<f64>,
    {
        let width = width.into() as f32;
        let height = height.into() as f32;

        Self {
            meta: RootMeta { width, height },
            style: Style {
                size: Size {
                    width: length(width),
                    height: length(height),
                },
                ..Default::default()
            },
        }
    }

    pub fn build(&self) -> Node {
        Node::new(NodeKind::Root(self.meta.to_owned()), self.style.to_owned())
    }
}
