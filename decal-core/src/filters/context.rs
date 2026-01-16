use crate::filters::primitives::{
    Blend, ColorMatrix, ComponentTransfer, Composite, ConvolveMatrix, DiffuseLighting,
    DisplacementMap, DropShadow, FilterPrimitive, Flood, GaussianBlur, Image, Merge, Morphology,
    Offset, PrimitiveBuilder, SpecularLighting, Tile, Turbulence,
};
use crate::paint::{Iri, ResourceIri};
use crate::primitives::LightSource;
use hashbrown::HashMap;
use parking_lot::Mutex;

#[derive(Debug, Copy, Clone)]
pub struct PrimitiveNode {
    #[allow(dead_code)]
    idx: usize,
    iri: Iri,
}

impl PrimitiveNode {
    pub(crate) fn iri(&self) -> Iri {
        self.iri
    }
}

#[derive(Debug, Default)]
struct ContextInner {
    index_map: HashMap<FilterPrimitive, usize>,
    primitives: Vec<FilterPrimitive>,
}

#[derive(Debug, Default)]
pub struct FilterContext(Mutex<ContextInner>);

impl<'a> FilterContext {
    pub fn flood(&self) -> PrimitiveBuilder<'_, Flood> {
        PrimitiveBuilder::new(self, Flood::new())
    }

    pub fn image(&self, href: &str) -> PrimitiveBuilder<'_, Image> {
        PrimitiveBuilder::new(self, Image::new(href))
    }

    pub fn gaussian_blur(&self) -> PrimitiveBuilder<'_, GaussianBlur> {
        PrimitiveBuilder::new(self, GaussianBlur::new())
    }

    pub fn turbulence(&self) -> PrimitiveBuilder<'_, Turbulence> {
        PrimitiveBuilder::new(self, Turbulence::new())
    }

    pub fn color_matrix(&self) -> PrimitiveBuilder<'_, ColorMatrix> {
        PrimitiveBuilder::new(self, ColorMatrix::new())
    }

    pub fn component_transfer(&self) -> PrimitiveBuilder<'_, ComponentTransfer> {
        PrimitiveBuilder::new(self, ComponentTransfer::new())
    }

    pub fn displacement_map(&self) -> PrimitiveBuilder<'_, DisplacementMap> {
        PrimitiveBuilder::new(self, DisplacementMap::new())
    }

    pub fn composite(&self) -> PrimitiveBuilder<'_, Composite> {
        PrimitiveBuilder::new(self, Composite::new())
    }

    pub fn blend(&self) -> PrimitiveBuilder<'_, Blend> {
        PrimitiveBuilder::new(self, Blend::new())
    }

    pub fn merge(&self) -> PrimitiveBuilder<'_, Merge> {
        PrimitiveBuilder::new(self, Merge::new())
    }

    pub fn specular_lighting(
        &self,
        light_source: LightSource,
    ) -> PrimitiveBuilder<'_, SpecularLighting> {
        PrimitiveBuilder::new(self, SpecularLighting::new(light_source))
    }

    pub fn diffuse_lighting(
        &self,
        light_source: LightSource,
    ) -> PrimitiveBuilder<'_, DiffuseLighting> {
        PrimitiveBuilder::new(self, DiffuseLighting::new(light_source))
    }

    pub fn convolve_matrix(&self, kernel_matrix: Vec<f32>) -> PrimitiveBuilder<'_, ConvolveMatrix> {
        PrimitiveBuilder::new(self, ConvolveMatrix::new(kernel_matrix))
    }

    pub fn drop_shadow(&self) -> PrimitiveBuilder<'_, DropShadow> {
        PrimitiveBuilder::new(self, DropShadow::new())
    }

    pub fn morphology(&self) -> PrimitiveBuilder<'_, Morphology> {
        PrimitiveBuilder::new(self, Morphology::new())
    }

    pub fn offset(&self) -> PrimitiveBuilder<'_, Offset> {
        PrimitiveBuilder::new(self, Offset::new())
    }

    pub fn tile(&self) -> PrimitiveBuilder<'_, Tile> {
        PrimitiveBuilder::new(self, Tile::new())
    }

    //

    pub(crate) fn into_primitives(self) -> Vec<FilterPrimitive> {
        self.0.into_inner().primitives
    }

    pub(crate) fn get_or_add_primitive(&self, primitive: FilterPrimitive) -> PrimitiveNode {
        let mut inner = self.0.lock();
        let iri = primitive.iri();

        if let Some(idx) = inner.index_map.get(&primitive) {
            return PrimitiveNode { idx: *idx, iri };
        }

        let idx = inner.primitives.len();
        inner.index_map.insert(primitive.clone(), idx);
        inner.primitives.push(primitive);

        PrimitiveNode { idx, iri }
    }
}
