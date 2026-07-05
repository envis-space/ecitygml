use crate::model::building::building_part_property::BuildingPartProperty;
use crate::model::building::{AbstractBuilding, AsAbstractBuilding, AsAbstractBuildingMut};
use crate::model::core::AsAbstractFeatureMut;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

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

impl Building {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::once(self.into())
            .chain(self.abstract_building.iter_features())
            .chain(
                self.building_parts
                    .iter()
                    .filter_map(|x| x.object.as_ref())
                    .flat_map(|x| x.iter_features()),
            )
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_building.for_each_feature_mut(f);

        for prop in &mut self.building_parts {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_building.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_building.apply_transform(m);

        for prop in &mut self.building_parts {
            if let Some(x) = prop.object.as_mut() {
                x.apply_transform(m);
            }
        }
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
