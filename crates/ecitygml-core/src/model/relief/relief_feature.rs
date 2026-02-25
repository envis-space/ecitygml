use crate::impl_abstract_space_boundary_traits;
use crate::model::common::LevelOfDetail;
use crate::model::core::{
    AbstractSpaceBoundary, AsAbstractFeatureMut, AsAbstractSpaceBoundary,
    AsAbstractSpaceBoundaryMut, CityObjectKind, CityObjectRef,
};
use crate::model::relief::ReliefComponentKind;
use crate::operations::{Visitable, Visitor};
use egml::model::geometry::Envelope;

#[derive(Debug, Clone, PartialEq)]
pub struct ReliefFeature {
    pub(crate) abstract_space_boundary: AbstractSpaceBoundary,
    lod: LevelOfDetail,
    relief_component: Vec<ReliefComponentKind>,
}

impl ReliefFeature {
    pub fn new(abstract_space_boundary: AbstractSpaceBoundary, lod: LevelOfDetail) -> Self {
        Self {
            abstract_space_boundary,
            lod,
            relief_component: Vec::new(),
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::ReliefFeature(self))
    }

    pub fn refresh_bounded_by_recursive(&mut self) {
        self.relief_component
            .iter_mut()
            .for_each(|x| x.refresh_bounded_by_recursive());

        let envelopes: Vec<&Envelope> = self
            .relief_component
            .iter()
            .filter_map(|x| x.bounded_by())
            .collect();

        self.set_bounded_by(Envelope::from_envelopes(&envelopes));
    }

    pub fn relief_component(&self) -> &[ReliefComponentKind] {
        &self.relief_component
    }

    pub fn relief_component_mut(&mut self) -> &mut Vec<ReliefComponentKind> {
        &mut self.relief_component
    }

    pub fn num_relief_components(&self) -> usize {
        self.relief_component.len()
    }

    pub fn lod(&self) -> LevelOfDetail {
        self.lod
    }
}

impl AsAbstractSpaceBoundary for ReliefFeature {
    fn abstract_space_boundary(&self) -> &AbstractSpaceBoundary {
        &self.abstract_space_boundary
    }
}

impl AsAbstractSpaceBoundaryMut for ReliefFeature {
    fn abstract_space_boundary_mut(&mut self) -> &mut AbstractSpaceBoundary {
        &mut self.abstract_space_boundary
    }
}

impl_abstract_space_boundary_traits!(ReliefFeature);

impl From<ReliefFeature> for CityObjectKind {
    fn from(item: ReliefFeature) -> Self {
        CityObjectKind::ReliefFeature(item)
    }
}

impl Visitable for ReliefFeature {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_relief_feature(self);
    }
}
