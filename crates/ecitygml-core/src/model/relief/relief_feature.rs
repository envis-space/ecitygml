use crate::impl_abstract_space_boundary_traits;
use crate::model::common::{FeatureRef, FeatureRefMut, LevelOfDetail, TopLevelFeatureRef};
use crate::model::core::{
    AbstractSpaceBoundary, AsAbstractFeatureMut, AsAbstractSpaceBoundary,
    AsAbstractSpaceBoundaryMut,
};
use crate::model::relief::ReliefComponentProperty;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct ReliefFeature {
    pub(crate) abstract_space_boundary: AbstractSpaceBoundary,
    lod: LevelOfDetail,
    relief_components: Vec<ReliefComponentProperty>,
}

impl ReliefFeature {
    pub fn new(abstract_space_boundary: AbstractSpaceBoundary, lod: LevelOfDetail) -> Self {
        Self {
            abstract_space_boundary,
            lod,
            relief_components: Vec::new(),
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        std::iter::once(self.into())
            .chain(self.abstract_space_boundary.iter_features())
            .chain(
                self.relief_components
                    .iter()
                    .flat_map(|x| x.object.as_ref())
                    .flat_map(|x| x.iter_features()),
            )
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_space_boundary.for_each_feature_mut(f);
        for prop in &mut self.relief_components {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_space_boundary.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_space_boundary.apply_transform(m);

        self.relief_components
            .iter_mut()
            .flat_map(|x| x.object.as_mut())
            .for_each(|x| x.apply_transform(m));
    }

    pub fn relief_components(&self) -> &[ReliefComponentProperty] {
        &self.relief_components
    }

    pub fn relief_components_mut(&mut self) -> &mut Vec<ReliefComponentProperty> {
        &mut self.relief_components
    }

    pub fn num_relief_components(&self) -> usize {
        self.relief_components.len()
    }

    pub fn set_relief_components(&mut self, values: Vec<ReliefComponentProperty>) {
        self.relief_components = values;
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

impl<'a> From<&'a ReliefFeature> for FeatureRef<'a> {
    fn from(item: &'a ReliefFeature) -> Self {
        FeatureRef::ReliefFeature(item)
    }
}

impl<'a> From<&'a mut ReliefFeature> for FeatureRefMut<'a> {
    fn from(item: &'a mut ReliefFeature) -> Self {
        FeatureRefMut::ReliefFeature(item)
    }
}

impl<'a> From<&'a ReliefFeature> for TopLevelFeatureRef<'a> {
    fn from(item: &'a ReliefFeature) -> Self {
        TopLevelFeatureRef::ReliefFeature(item)
    }
}
