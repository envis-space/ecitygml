use crate::impl_abstract_building_subdivision_traits;
use crate::model::building::{
    AbstractBuildingSubdivision, AsAbstractBuildingSubdivision, AsAbstractBuildingSubdivisionMut,
};
use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::core::AsAbstractFeatureMut;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingUnit {
    pub abstract_building_subdivision: AbstractBuildingSubdivision,
}

impl BuildingUnit {
    pub fn new(abstract_building_subdivision: AbstractBuildingSubdivision) -> Self {
        Self {
            abstract_building_subdivision,
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_building_subdivision.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_building_subdivision.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_building_subdivision.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_building_subdivision.apply_transform(m);
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

impl<'a> From<&'a BuildingUnit> for FeatureRef<'a> {
    fn from(item: &'a BuildingUnit) -> Self {
        FeatureRef::BuildingUnit(item)
    }
}

impl<'a> From<&'a mut BuildingUnit> for FeatureRefMut<'a> {
    fn from(item: &'a mut BuildingUnit) -> Self {
        FeatureRefMut::BuildingUnit(item)
    }
}
