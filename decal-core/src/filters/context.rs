use crate::filters::primitives::{
    Blend, Blur, ColorMatrix, DisplacementMap, Flood, Merge, PrimitiveBuilder, SpecularLighting,
    Turbulence,
};
use crate::filters::primitives::{ComponentTransfer, FilterPrimitive};
use crate::filters::primitives::{Composite, Image};
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

    pub fn blur(&self) -> PrimitiveBuilder<'_, Blur> {
        PrimitiveBuilder::new(self, Blur::new())
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
