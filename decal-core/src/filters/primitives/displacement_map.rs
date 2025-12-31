use crate::filters::primitives::PrimitiveBuilder;
use crate::filters::{FilterRegion, HasFilterRegion};
use crate::macros::ff32;
use crate::paint::ResourceIri;
use crate::primitives::FilterInput;
use crate::utils::{FloatWriter, IsDefault};
use enum_display::EnumDisplay;
use std::fmt::{Display, Formatter, Write};
use strict_num::FiniteF32;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum ChannelSelector {
    #[display("R")]
    R,
    #[display("G")]
    G,
    #[display("B")]
    B,
    #[default]
    #[display("A")]
    A,
}

impl IsDefault for ChannelSelector {}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
pub struct DisplacementMap {
    input: Option<FilterInput>,
    map: Option<FilterInput>,
    scale: FiniteF32,
    x_channel_selector: ChannelSelector,
    y_channel_selector: ChannelSelector,
    region: FilterRegion,
}

impl DisplacementMap {
    pub(crate) fn new() -> Self {
        DisplacementMap::default()
    }
}

impl ResourceIri for DisplacementMap {}

impl HasFilterRegion for DisplacementMap {
    fn region_mut(&mut self) -> &mut FilterRegion {
        &mut self.region
    }
}

impl Display for DisplacementMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<feDisplacementMap")?;
        self.region.fmt(f)?;

        if let Some(input) = self.input {
            write!(f, r#" in="{input}""#)?;
        }

        if let Some(map) = self.map {
            write!(f, r#" in2="{map}""#)?;
        }

        if self.scale.get() != 0.0 {
            f.write_str(r#" scale=""#)?;
            f.write_float(self.scale.get())?;
            f.write_char('"')?;
        }

        if !self.x_channel_selector.is_default() {
            write!(f, r#" xChannelSelector="{}""#, self.x_channel_selector)?;
        }

        if !self.y_channel_selector.is_default() {
            write!(f, r#" yChannelSelector="{}""#, self.y_channel_selector)?;
        }

        write!(f, r#" result="{}" />"#, self.iri())
    }
}

impl<'a> PrimitiveBuilder<'a, DisplacementMap> {
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.input = Some(input.into());
        self
    }

    pub fn map<T>(mut self, input: T) -> Self
    where
        T: Into<FilterInput>,
    {
        self.inner.map = Some(input.into());
        self
    }

    pub fn scale<T>(mut self, scale: T) -> Self
    where
        T: Into<f32>,
    {
        self.inner.scale = ff32!(scale);
        self
    }

    pub fn x_channel(mut self, channel_selector: ChannelSelector) -> Self {
        self.inner.x_channel_selector = channel_selector;
        self
    }

    pub fn y_channel(mut self, channel_selector: ChannelSelector) -> Self {
        self.inner.y_channel_selector = channel_selector;
        self
    }
}
