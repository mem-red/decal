use crate::filters::primitives::{
    Blend, ColorMatrix, ComponentTransfer, Composite, ConvolveMatrix, DiffuseLighting,
    DisplacementMap, DropShadow, Flood, GaussianBlur, Image, Merge, Morphology, Offset,
    SpecularLighting, Tile, Turbulence,
};
use crate::paint::{Iri, ResourceIri};
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum FilterPrimitive {
    Flood(Flood),
    Image(Image),
    GaussianBlur(GaussianBlur),
    Turbulence(Turbulence),
    ColorMatrix(ColorMatrix),
    ComponentTransfer(ComponentTransfer),
    DisplacementMap(DisplacementMap),
    Composite(Composite),
    Blend(Blend),
    Merge(Merge),
    SpecularLighting(SpecularLighting),
    DiffuseLighting(DiffuseLighting),
    ConvolveMatrix(ConvolveMatrix),
    DropShadow(DropShadow),
    Morphology(Morphology),
    Offset(Offset),
    Tile(Tile),
}

macro_rules! delegate_primitive {
    ($this:expr, $method:ident($($arg:expr),*)) => {
        match $this {
            Self::Flood(x) => x.$method($($arg),*),
            Self::Image(x) => x.$method($($arg),*),
            Self::GaussianBlur(x) => x.$method($($arg),*),
            Self::Turbulence(x) => x.$method($($arg),*),
            Self::ColorMatrix(x) => x.$method($($arg),*),
            Self::ComponentTransfer(x) => x.$method($($arg),*),
            Self::DisplacementMap(x) => x.$method($($arg),*),
            Self::Composite(x) => x.$method($($arg),*),
            Self::Blend(x) => x.$method($($arg),*),
            Self::Merge(x) => x.$method($($arg),*),
            Self::SpecularLighting(x) => x.$method($($arg),*),
            Self::DiffuseLighting(x) => x.$method($($arg),*),
            Self::ConvolveMatrix(x) => x.$method($($arg),*),
            Self::DropShadow(x) => x.$method($($arg),*),
            Self::Morphology(x) => x.$method($($arg),*),
            Self::Offset(x) => x.$method($($arg),*),
            Self::Tile(x) => x.$method($($arg),*),
        }
    };
}

impl Display for FilterPrimitive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        delegate_primitive!(self, fmt(f))
    }
}

impl ResourceIri for FilterPrimitive {
    fn iri(&self) -> Iri {
        delegate_primitive!(self, iri())
    }
}

//

impl From<Flood> for FilterPrimitive {
    fn from(value: Flood) -> Self {
        Self::Flood(value)
    }
}

impl From<Image> for FilterPrimitive {
    fn from(value: Image) -> Self {
        Self::Image(value)
    }
}

impl From<GaussianBlur> for FilterPrimitive {
    fn from(value: GaussianBlur) -> Self {
        Self::GaussianBlur(value)
    }
}

impl From<Turbulence> for FilterPrimitive {
    fn from(value: Turbulence) -> Self {
        Self::Turbulence(value)
    }
}

impl From<ColorMatrix> for FilterPrimitive {
    fn from(value: ColorMatrix) -> Self {
        Self::ColorMatrix(value)
    }
}

impl From<ComponentTransfer> for FilterPrimitive {
    fn from(value: ComponentTransfer) -> Self {
        Self::ComponentTransfer(value)
    }
}

impl From<DisplacementMap> for FilterPrimitive {
    fn from(value: DisplacementMap) -> Self {
        Self::DisplacementMap(value)
    }
}

impl From<Composite> for FilterPrimitive {
    fn from(value: Composite) -> Self {
        Self::Composite(value)
    }
}

impl From<Blend> for FilterPrimitive {
    fn from(value: Blend) -> Self {
        Self::Blend(value)
    }
}

impl From<Merge> for FilterPrimitive {
    fn from(value: Merge) -> Self {
        Self::Merge(value)
    }
}

impl From<SpecularLighting> for FilterPrimitive {
    fn from(value: SpecularLighting) -> Self {
        Self::SpecularLighting(value)
    }
}

impl From<DiffuseLighting> for FilterPrimitive {
    fn from(value: DiffuseLighting) -> Self {
        Self::DiffuseLighting(value)
    }
}

impl From<ConvolveMatrix> for FilterPrimitive {
    fn from(value: ConvolveMatrix) -> Self {
        Self::ConvolveMatrix(value)
    }
}

impl From<DropShadow> for FilterPrimitive {
    fn from(value: DropShadow) -> Self {
        Self::DropShadow(value)
    }
}

impl From<Morphology> for FilterPrimitive {
    fn from(value: Morphology) -> Self {
        Self::Morphology(value)
    }
}

impl From<Offset> for FilterPrimitive {
    fn from(value: Offset) -> Self {
        Self::Offset(value)
    }
}

impl From<Tile> for FilterPrimitive {
    fn from(value: Tile) -> Self {
        Self::Tile(value)
    }
}
