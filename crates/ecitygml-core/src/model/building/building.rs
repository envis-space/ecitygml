use crate::model::building::{AbstractBuilding, AsAbstractBuilding, AsAbstractBuildingMut};
use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};
use crate::model::core::AsAbstractFeatureMut;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct Building {
    pub abstract_building: AbstractBuilding,
}

impl Building {
    pub fn new(abstract_building: AbstractBuilding) -> Self {
        Self { abstract_building }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_building.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_building.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_building.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_building.apply_transform(m);
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

impl<'a> From<&'a Building> for FeatureRef<'a> {
    fn from(item: &'a Building) -> Self {
        FeatureRef::Building(item)
    }
}

impl<'a> From<&'a mut Building> for FeatureRefMut<'a> {
    fn from(item: &'a mut Building) -> Self {
        FeatureRefMut::Building(item)
    }
}

impl<'a> From<&'a Building> for TopLevelFeatureRef<'a> {
    fn from(item: &'a Building) -> Self {
        TopLevelFeatureRef::Building(item)
    }
}
