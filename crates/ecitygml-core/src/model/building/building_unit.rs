use crate::impl_abstract_building_subdivision_mut_traits;
use crate::impl_abstract_building_subdivision_traits;
use crate::model::building::{
    AbstractBuildingSubdivision, AsAbstractBuildingSubdivision, AsAbstractBuildingSubdivisionMut,
};
use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingUnit {
    pub(crate) abstract_building_subdivision: AbstractBuildingSubdivision,
}

impl BuildingUnit {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_building_subdivision(AbstractBuildingSubdivision::new(id))
    }

    pub fn from_abstract_building_subdivision(
        abstract_building_subdivision: AbstractBuildingSubdivision,
    ) -> Self {
        Self {
            abstract_building_subdivision,
        }
    }
}

impl AsAbstractBuildingSubdivision for BuildingUnit {
    fn abstract_building_subdivision(&self) -> &AbstractBuildingSubdivision {
        &self.abstract_building_subdivision
    }
}

impl AsAbstractBuildingSubdivisionMut for BuildingUnit {
    fn abstract_building_subdivision_mut(&mut self) -> &mut AbstractBuildingSubdivision {
        &mut self.abstract_building_subdivision
    }
}

impl_abstract_building_subdivision_traits!(BuildingUnit);
impl_abstract_building_subdivision_mut_traits!(BuildingUnit);
crate::impl_has_feature_type!(BuildingUnit, BuildingUnit);

impl IterFeatures for BuildingUnit {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into()).chain(self.abstract_building_subdivision.iter_features()),
        )
    }
}

impl ForEachFeatureMut for BuildingUnit {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_building_subdivision.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for BuildingUnit {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_building_subdivision.compute_envelope()
    }
}

impl ApplyTransform for BuildingUnit {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_building_subdivision.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_building_subdivision.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_building_subdivision.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_building_subdivision.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_building_subdivision.apply_scale(scale);
    }
}
