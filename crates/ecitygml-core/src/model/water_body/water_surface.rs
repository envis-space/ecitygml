use crate::model::core::AsAbstractFeatureMut;
use crate::model::core::refs::{FeatureKindRef, FeatureKindRefMut};
use crate::model::water_body::{
    AbstractWaterBoundarySurface, AsAbstractWaterBoundarySurface, AsAbstractWaterBoundarySurfaceMut,
};
use egml::model::base::Id;
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct WaterSurface {
    pub(crate) abstract_water_boundary_surface: AbstractWaterBoundarySurface,
    water_level: Option<Code>,
}

impl WaterSurface {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_water_boundary_surface(AbstractWaterBoundarySurface::new(id))
    }

    pub fn from_abstract_water_boundary_surface(
        abstract_water_boundary_surface: AbstractWaterBoundarySurface,
    ) -> Self {
        Self {
            abstract_water_boundary_surface,
            water_level: None,
        }
    }
}

impl WaterSurface {
    pub fn water_level(&self) -> Option<&Code> {
        self.water_level.as_ref()
    }

    pub fn set_water_level(&mut self, water_level: Option<Code>) {
        self.water_level = water_level;
    }
}

impl WaterSurface {
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

impl AsAbstractWaterBoundarySurface for WaterSurface {
    fn abstract_water_boundary_surface(&self) -> &AbstractWaterBoundarySurface {
        &self.abstract_water_boundary_surface
    }
}

impl AsAbstractWaterBoundarySurfaceMut for WaterSurface {
    fn abstract_water_boundary_surface_mut(&mut self) -> &mut AbstractWaterBoundarySurface {
        &mut self.abstract_water_boundary_surface
    }
}

crate::impl_abstract_water_boundary_surface_traits!(WaterSurface);
crate::impl_abstract_water_boundary_surface_mut_traits!(WaterSurface);
crate::impl_has_feature_type!(WaterSurface, WaterSurface);
