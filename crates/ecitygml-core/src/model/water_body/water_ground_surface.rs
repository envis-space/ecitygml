use crate::model::core::AsAbstractFeatureMut;
use crate::model::core::refs::{FeatureKindRef, FeatureKindRefMut};
use crate::model::water_body::{
    AbstractWaterBoundarySurface, AsAbstractWaterBoundarySurface, AsAbstractWaterBoundarySurfaceMut,
};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct WaterGroundSurface {
    pub(crate) abstract_water_boundary_surface: AbstractWaterBoundarySurface,
}

impl WaterGroundSurface {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_water_boundary_surface(AbstractWaterBoundarySurface::new(id))
    }

    pub fn from_abstract_water_boundary_surface(
        abstract_water_boundary_surface: AbstractWaterBoundarySurface,
    ) -> Self {
        Self {
            abstract_water_boundary_surface,
        }
    }
}
impl WaterGroundSurface {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_water_boundary_surface.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_water_boundary_surface.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_water_boundary_surface.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_water_boundary_surface.apply_transform(m);
    }
}

impl AsAbstractWaterBoundarySurface for WaterGroundSurface {
    fn abstract_water_boundary_surface(&self) -> &AbstractWaterBoundarySurface {
        &self.abstract_water_boundary_surface
    }
}

impl AsAbstractWaterBoundarySurfaceMut for WaterGroundSurface {
    fn abstract_water_boundary_surface_mut(&mut self) -> &mut AbstractWaterBoundarySurface {
        &mut self.abstract_water_boundary_surface
    }
}

crate::impl_abstract_water_boundary_surface_traits!(WaterGroundSurface);
crate::impl_abstract_water_boundary_surface_mut_traits!(WaterGroundSurface);
crate::impl_has_feature_type!(WaterGroundSurface, WaterGroundSurface);
