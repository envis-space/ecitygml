use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};
use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractFeatureMut, AsAbstractOccupiedSpace,
    AsAbstractOccupiedSpaceMut,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct CityFurniture {
    pub abstract_occupied_space: AbstractOccupiedSpace,
}

impl CityFurniture {
    pub fn new(abstract_occupied_space: AbstractOccupiedSpace) -> Self {
        Self {
            abstract_occupied_space,
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_occupied_space.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_occupied_space.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_occupied_space.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_occupied_space.apply_transform(m);
    }
}

impl AsAbstractOccupiedSpace for CityFurniture {
    fn abstract_occupied_space(&self) -> &AbstractOccupiedSpace {
        &self.abstract_occupied_space
    }
}

impl AsAbstractOccupiedSpaceMut for CityFurniture {
    fn abstract_occupied_space_mut(&mut self) -> &mut AbstractOccupiedSpace {
        &mut self.abstract_occupied_space
    }
}

crate::impl_abstract_occupied_space_traits!(CityFurniture);

impl<'a> From<&'a CityFurniture> for FeatureRef<'a> {
    fn from(item: &'a CityFurniture) -> Self {
        FeatureRef::CityFurniture(item)
    }
}

impl<'a> From<&'a mut CityFurniture> for FeatureRefMut<'a> {
    fn from(item: &'a mut CityFurniture) -> Self {
        FeatureRefMut::CityFurniture(item)
    }
}

impl<'a> From<&'a CityFurniture> for TopLevelFeatureRef<'a> {
    fn from(item: &'a CityFurniture) -> Self {
        TopLevelFeatureRef::CityFurniture(item)
    }
}
