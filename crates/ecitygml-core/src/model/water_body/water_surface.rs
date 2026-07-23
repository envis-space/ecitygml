use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::{AbstractFeatureKindRef, AbstractFeatureKindRefMut};
use crate::model::water_body::values::WaterLevelValue;
use crate::model::water_body::{
    AbstractWaterBoundarySurface, AsAbstractWaterBoundarySurface, AsAbstractWaterBoundarySurfaceMut,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct WaterSurface {
    pub(crate) abstract_water_boundary_surface: AbstractWaterBoundarySurface,
    water_level: Option<WaterLevelValue>,
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
    pub fn water_level(&self) -> Option<&WaterLevelValue> {
        self.water_level.as_ref()
    }

    pub fn set_water_level(&mut self, water_level: WaterLevelValue) {
        self.water_level = Some(water_level);
    }

    pub fn set_water_level_opt(&mut self, water_level: Option<WaterLevelValue>) {
        self.water_level = water_level;
    }

    pub fn clear_water_level(&mut self) {
        self.water_level = None;
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

impl IterFeatures for WaterSurface {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into())
                .chain(self.abstract_water_boundary_surface.iter_features()),
        )
    }
}

impl ForEachFeatureMut for WaterSurface {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_water_boundary_surface.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for WaterSurface {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_water_boundary_surface.compute_envelope()
    }
}

impl ApplyTransform for WaterSurface {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_water_boundary_surface.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_water_boundary_surface
            .apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_water_boundary_surface
            .apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_water_boundary_surface
            .apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_water_boundary_surface.apply_scale(scale);
    }
}
