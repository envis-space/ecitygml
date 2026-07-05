use crate::model::core::refs::{FeatureKindRef, FeatureKindRefMut};
use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

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
impl AbstractWaterBoundarySurface {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        self.abstract_thematic_surface.iter_features()
    }
    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_thematic_surface.for_each_feature_mut(f);
    }
    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_thematic_surface.compute_envelope()
    }
    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_thematic_surface.apply_transform(m);
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
                use $crate::model::water_body::AsAbstractWaterBoundarySurface;
                &self
                    .abstract_water_boundary_surface()
                    .abstract_thematic_surface
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
                use $crate::model::water_body::AsAbstractWaterBoundarySurfaceMut;
                &mut self
                    .abstract_water_boundary_surface_mut()
                    .abstract_thematic_surface
            }
        }
    };
}

impl_abstract_water_boundary_surface_traits!(AbstractWaterBoundarySurface);
impl_abstract_water_boundary_surface_mut_traits!(AbstractWaterBoundarySurface);
