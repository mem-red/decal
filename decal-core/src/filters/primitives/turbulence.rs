use crate::filters::primitives::PrimitiveBuilder;
use crate::paint::ResourceIri;
use crate::primitives::PositiveF32Pair;
use crate::utils::IsDefault;
use enum_display::EnumDisplay;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default, EnumDisplay)]
pub enum TurbulenceType {
    #[default]
    #[display("turbulence")]
    Turbulence,
    #[display("fractalNoise")]
    FractalNoise,
}

impl IsDefault for TurbulenceType {}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Turbulence {
    base_freq: PositiveF32Pair,
    num_octaves: u64,
    seed: u64,
    kind: TurbulenceType,
}

impl Default for Turbulence {
    fn default() -> Self {
        Self {
            base_freq: Default::default(),
            num_octaves: 1,
            seed: 0,
            kind: Default::default(),
        }
    }
}

impl Turbulence {
    pub(crate) fn new() -> Self {
        Turbulence::default()
    }
}

impl ResourceIri for Turbulence {}

impl Display for Turbulence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<feTurbulence")?;

        if !self.kind.is_default() {
            write!(f, r#" type="{}""#, self.kind)?;
        }

        if self.seed != 0 {
            write!(f, r#" seed="{}""#, self.seed)?;
        }

        if !self.base_freq.is_zero() {
            write!(f, r#" baseFrequency="{}""#, self.base_freq)?;
        }

        if self.num_octaves != 1 {
            write!(f, r#" numOctaves="{}""#, self.num_octaves)?;
        }

        write!(f, r#" result="{}" />"#, self.iri())
    }
}

impl<'a> PrimitiveBuilder<'a, Turbulence> {
    pub fn base_freq<T>(mut self, base_freq: T) -> Self
    where
        T: Into<PositiveF32Pair>,
    {
        self.inner.base_freq = base_freq.into();
        self
    }

    pub fn num_octaves(mut self, num_octaves: u64) -> Self {
        self.inner.num_octaves = num_octaves;
        self
    }

    pub fn seed(mut self, seed: u64) -> Self {
        self.inner.seed = seed;
        self
    }

    pub fn kind(mut self, kind: TurbulenceType) -> Self {
        self.inner.kind = kind;
        self
    }

    pub fn fractal_noise(mut self) -> Self {
        self.inner.kind = TurbulenceType::FractalNoise;
        self
    }
}
