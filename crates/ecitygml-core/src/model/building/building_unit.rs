use crate::impl_abstract_building_subdivision_mut_traits;
use crate::impl_abstract_building_subdivision_traits;
use crate::model::building::{
    AbstractBuildingSubdivision, AsAbstractBuildingSubdivision, AsAbstractBuildingSubdivisionMut,
};
use crate::model::core::AsAbstractFeatureMut;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

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
impl BuildingUnit {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_building_subdivision.iter_features())
    }
    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
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
impl_abstract_building_subdivision_mut_traits!(BuildingUnit);
crate::impl_has_feature_type!(BuildingUnit, BuildingUnit);
