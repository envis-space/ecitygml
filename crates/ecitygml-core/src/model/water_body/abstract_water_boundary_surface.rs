use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::{AbstractFeatureKindRef, AbstractFeatureKindRefMut};
use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractWaterBoundarySurface {
    pub(crate) abstract_thematic_surface: AbstractThematicSurface,
}

impl AbstractWaterBoundarySurface {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_thematic_surface(AbstractThematicSurface::new(id))
    }

    pub fn from_abstract_thematic_surface(
        abstract_thematic_surface: AbstractThematicSurface,
    ) -> Self {
        Self {
            abstract_thematic_surface,
        }
    }
}

pub trait AsAbstractWaterBoundarySurface: AsAbstractThematicSurface {
    fn abstract_water_boundary_surface(&self) -> &AbstractWaterBoundarySurface;
}

pub trait AsAbstractWaterBoundarySurfaceMut:
    AsAbstractThematicSurfaceMut + AsAbstractWaterBoundarySurface
{
    fn abstract_water_boundary_surface_mut(&mut self) -> &mut AbstractWaterBoundarySurface;
}

impl AsAbstractWaterBoundarySurface for AbstractWaterBoundarySurface {
    fn abstract_water_boundary_surface(&self) -> &AbstractWaterBoundarySurface {
        self
    }
}

impl AsAbstractWaterBoundarySurfaceMut for AbstractWaterBoundarySurface {
    fn abstract_water_boundary_surface_mut(&mut self) -> &mut AbstractWaterBoundarySurface {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_water_boundary_surface_traits {
    ($type:ty) => {
        $crate::impl_abstract_thematic_surface_traits!($type);

        impl $crate::model::core::AsAbstractThematicSurface for $type {
            fn abstract_thematic_surface(&self) -> &$crate::model::core::AbstractThematicSurface {
                &<$type as $crate::model::water_body::AsAbstractWaterBoundarySurface>::abstract_water_boundary_surface(self).abstract_thematic_surface
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_water_boundary_surface_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_thematic_surface_mut_traits!($type);

        impl $crate::model::core::AsAbstractThematicSurfaceMut for $type {
            fn abstract_thematic_surface_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractThematicSurface {
                &mut <$type as $crate::model::water_body::AsAbstractWaterBoundarySurfaceMut>::abstract_water_boundary_surface_mut(self).abstract_thematic_surface
            }
        }
    };
}

impl_abstract_water_boundary_surface_traits!(AbstractWaterBoundarySurface);
impl_abstract_water_boundary_surface_mut_traits!(AbstractWaterBoundarySurface);

impl IterFeatures for AbstractWaterBoundarySurface {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        self.abstract_thematic_surface.iter_features()
    }
}

impl ForEachFeatureMut for AbstractWaterBoundarySurface {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_thematic_surface.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for AbstractWaterBoundarySurface {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_thematic_surface.compute_envelope()
    }
}

impl ApplyTransform for AbstractWaterBoundarySurface {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_thematic_surface.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_thematic_surface.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_thematic_surface.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_thematic_surface.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_thematic_surface.apply_scale(scale);
    }
}
