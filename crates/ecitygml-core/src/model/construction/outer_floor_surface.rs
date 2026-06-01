use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::construction::{
    AbstractConstructionSurface, AsAbstractConstructionSurface, AsAbstractConstructionSurfaceMut,
};
use crate::model::core::AsAbstractFeatureMut;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct OuterFloorSurface {
    pub abstract_construction_surface: AbstractConstructionSurface,
}

impl OuterFloorSurface {
    pub fn new(abstract_construction_surface: AbstractConstructionSurface) -> Self {
        Self {
            abstract_construction_surface,
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_construction_surface.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_construction_surface.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_construction_surface.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_construction_surface.apply_transform(m);
    }
}

impl AsAbstractConstructionSurface for OuterFloorSurface {
    fn abstract_construction_surface(&self) -> &AbstractConstructionSurface {
        &self.abstract_construction_surface
    }
}

impl AsAbstractConstructionSurfaceMut for OuterFloorSurface {
    fn abstract_construction_surface_mut(&mut self) -> &mut AbstractConstructionSurface {
        &mut self.abstract_construction_surface
    }
}

crate::impl_abstract_construction_surface_traits!(OuterFloorSurface);

impl<'a> From<&'a OuterFloorSurface> for FeatureRef<'a> {
    fn from(item: &'a OuterFloorSurface) -> Self {
        FeatureRef::OuterFloorSurface(item)
    }
}

impl<'a> From<&'a mut OuterFloorSurface> for FeatureRefMut<'a> {
    fn from(item: &'a mut OuterFloorSurface) -> Self {
        FeatureRefMut::OuterFloorSurface(item)
    }
}
