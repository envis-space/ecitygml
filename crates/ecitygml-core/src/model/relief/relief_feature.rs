use crate::impl_abstract_space_boundary_mut_traits;
use crate::impl_abstract_space_boundary_traits;
use crate::model::common::{ForEachFeatureMut, IterFeatures, LevelOfDetail};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractSpaceBoundary, AsAbstractSpaceBoundary, AsAbstractSpaceBoundaryMut,
};
use crate::model::relief::AbstractReliefComponentProperty;
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct ReliefFeature {
    pub(crate) abstract_space_boundary: AbstractSpaceBoundary,
    lod: LevelOfDetail,
    relief_components: Vec<AbstractReliefComponentProperty>,
}

impl ReliefFeature {
    pub fn new(id: Id, lod: LevelOfDetail) -> Self {
        Self::from_abstract_space_boundary(AbstractSpaceBoundary::new(id), lod)
    }

    pub fn from_abstract_space_boundary(
        abstract_space_boundary: AbstractSpaceBoundary,
        lod: LevelOfDetail,
    ) -> Self {
        Self {
            abstract_space_boundary,
            lod,
            relief_components: Vec::new(),
        }
    }

    pub fn relief_components(&self) -> &[AbstractReliefComponentProperty] {
        &self.relief_components
    }

    pub fn set_relief_components(&mut self, values: Vec<AbstractReliefComponentProperty>) {
        self.relief_components = values;
    }

    pub fn lod(&self) -> LevelOfDetail {
        self.lod
    }
}

impl ReliefFeature {
    pub fn relief_components_mut(&mut self) -> &mut Vec<AbstractReliefComponentProperty> {
        &mut self.relief_components
    }

    pub fn num_relief_components(&self) -> usize {
        self.relief_components.len()
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
impl_abstract_space_boundary_mut_traits!(ReliefFeature);
crate::impl_has_feature_type!(ReliefFeature, ReliefFeature);

impl IterFeatures for ReliefFeature {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into())
                .chain(self.abstract_space_boundary.iter_features())
                .chain(
                    self.relief_components
                        .iter()
                        .flat_map(|x| x.object())
                        .flat_map(|x| x.iter_features()),
                ),
        )
    }
}

impl ForEachFeatureMut for ReliefFeature {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_space_boundary.for_each_feature_mut(f);
        for prop in &mut self.relief_components {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }
}

impl ComputeEnvelope for ReliefFeature {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_space_boundary.compute_envelope()
    }
}

impl ApplyTransform for ReliefFeature {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_space_boundary.apply_transform(m);

        self.relief_components
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_transform(m));
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_space_boundary.apply_isometry(isometry);

        self.relief_components
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_isometry(isometry));
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_space_boundary.apply_translation(vector);

        self.relief_components
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_translation(vector));
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_space_boundary.apply_rotation(rotation);

        self.relief_components
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_rotation(rotation));
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_space_boundary.apply_scale(scale);

        self.relief_components
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_scale(scale));
    }
}
