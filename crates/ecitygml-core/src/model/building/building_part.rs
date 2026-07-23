use crate::model::building::{AbstractBuilding, AsAbstractBuilding, AsAbstractBuildingMut};
use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingPart {
    pub(crate) abstract_building: AbstractBuilding,
}

impl BuildingPart {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_building(AbstractBuilding::new(id))
    }

    pub fn from_abstract_building(abstract_building: AbstractBuilding) -> Self {
        Self { abstract_building }
    }
}

impl AsAbstractBuilding for BuildingPart {
    fn abstract_building(&self) -> &AbstractBuilding {
        &self.abstract_building
    }
}

impl AsAbstractBuildingMut for BuildingPart {
    fn abstract_building_mut(&mut self) -> &mut AbstractBuilding {
        &mut self.abstract_building
    }
}

crate::impl_abstract_building_traits!(BuildingPart);
crate::impl_abstract_building_mut_traits!(BuildingPart);
crate::impl_has_feature_type!(BuildingPart, BuildingPart);

impl IterFeatures for BuildingPart {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()).chain(self.abstract_building.iter_features()))
    }
}

impl ForEachFeatureMut for BuildingPart {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_building.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for BuildingPart {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_building.compute_envelope()
    }
}

impl ApplyTransform for BuildingPart {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_building.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_building.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_building.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_building.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_building.apply_scale(scale);
    }
}
