use crate::model::building::building_part_property::BuildingPartProperty;
use crate::model::building::{AbstractBuilding, AsAbstractBuilding, AsAbstractBuildingMut};
use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct Building {
    pub(crate) abstract_building: AbstractBuilding,
    building_parts: Vec<BuildingPartProperty>,
}

impl Building {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_building(AbstractBuilding::new(id))
    }

    pub fn from_abstract_building(abstract_building: AbstractBuilding) -> Self {
        Self {
            abstract_building,
            building_parts: Vec::new(),
        }
    }

    pub fn building_parts(&self) -> &[BuildingPartProperty] {
        &self.building_parts
    }

    pub fn building_parts_mut(&mut self) -> &mut [BuildingPartProperty] {
        &mut self.building_parts
    }

    pub fn set_building_parts(&mut self, building_parts: Vec<BuildingPartProperty>) {
        self.building_parts = building_parts;
    }

    pub fn push_building_part(&mut self, building_part: BuildingPartProperty) {
        self.building_parts.push(building_part);
    }

    pub fn extend_building_parts(
        &mut self,
        building_parts: impl IntoIterator<Item = BuildingPartProperty>,
    ) {
        self.building_parts.extend(building_parts);
    }
}

impl AsAbstractBuilding for Building {
    fn abstract_building(&self) -> &AbstractBuilding {
        &self.abstract_building
    }
}

impl AsAbstractBuildingMut for Building {
    fn abstract_building_mut(&mut self) -> &mut AbstractBuilding {
        &mut self.abstract_building
    }
}

crate::impl_abstract_building_traits!(Building);
crate::impl_abstract_building_mut_traits!(Building);
crate::impl_has_feature_type!(Building, Building);

impl IterFeatures for Building {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into())
                .chain(self.abstract_building.iter_features())
                .chain(
                    self.building_parts
                        .iter()
                        .filter_map(|x| x.object())
                        .flat_map(|x| x.iter_features()),
                ),
        )
    }
}

impl ForEachFeatureMut for Building {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_building.for_each_feature_mut(f);

        for prop in &mut self.building_parts {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }
}

impl ComputeEnvelope for Building {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_building.compute_envelope()
    }
}

impl ApplyTransform for Building {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_building.apply_transform(m);

        for prop in &mut self.building_parts {
            if let Some(x) = prop.object_mut() {
                x.apply_transform(m);
            }
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_building.apply_isometry(isometry);

        for prop in &mut self.building_parts {
            if let Some(x) = prop.object_mut() {
                x.apply_isometry(isometry);
            }
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_building.apply_translation(vector);

        for prop in &mut self.building_parts {
            if let Some(x) = prop.object_mut() {
                x.apply_translation(vector);
            }
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_building.apply_rotation(rotation);

        for prop in &mut self.building_parts {
            if let Some(x) = prop.object_mut() {
                x.apply_rotation(rotation);
            }
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_building.apply_scale(scale);

        for prop in &mut self.building_parts {
            if let Some(x) = prop.object_mut() {
                x.apply_scale(scale);
            }
        }
    }
}
