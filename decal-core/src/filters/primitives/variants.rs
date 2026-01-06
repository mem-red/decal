use crate::filters::primitives::{
    Blend, Blur, ColorMatrix, ComponentTransfer, Composite, ConvolveMatrix, DiffuseLighting,
    DisplacementMap, DropShadow, Flood, Image, Merge, SpecularLighting, Turbulence,
};
use crate::paint::{Iri, ResourceIri};
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum FilterPrimitive {
    Flood(Flood),
    Image(Image),
    Blur(Blur),
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
}

impl Display for FilterPrimitive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Flood(flood) => flood.fmt(f),
            Self::Image(image) => image.fmt(f),
            Self::Blur(blur) => blur.fmt(f),
            Self::Turbulence(turbulence) => turbulence.fmt(f),
            Self::ColorMatrix(matrix) => matrix.fmt(f),
            Self::ComponentTransfer(component_transfer) => component_transfer.fmt(f),
            Self::DisplacementMap(map) => map.fmt(f),
            Self::Composite(composite) => composite.fmt(f),
            Self::Blend(blend) => blend.fmt(f),
            Self::Merge(merge) => merge.fmt(f),
            Self::SpecularLighting(lighting) => lighting.fmt(f),
            Self::DiffuseLighting(lighting) => lighting.fmt(f),
            Self::ConvolveMatrix(matrix) => matrix.fmt(f),
            Self::DropShadow(shadow) => shadow.fmt(f),
        }
    }
}

impl ResourceIri for FilterPrimitive {
    fn iri(&self) -> Iri {
        match self {
            Self::Flood(flood) => flood.iri(),
            Self::Image(image) => image.iri(),
            Self::Blur(blur) => blur.iri(),
            Self::Turbulence(turbulence) => turbulence.iri(),
            Self::ColorMatrix(matrix) => matrix.iri(),
            Self::ComponentTransfer(component_transfer) => component_transfer.iri(),
            Self::DisplacementMap(map) => map.iri(),
            Self::Composite(composite) => composite.iri(),
            Self::Blend(blend) => blend.iri(),
            Self::Merge(merge) => merge.iri(),
            Self::SpecularLighting(lighting) => lighting.iri(),
            Self::DiffuseLighting(lighting) => lighting.iri(),
            Self::ConvolveMatrix(matrix) => matrix.iri(),
            Self::DropShadow(shadow) => shadow.iri(),
        }
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

impl From<Blur> for FilterPrimitive {
    fn from(value: Blur) -> Self {
        Self::Blur(value)
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
