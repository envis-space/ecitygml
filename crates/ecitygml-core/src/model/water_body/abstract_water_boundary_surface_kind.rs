use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::water_body::water_ground_surface::WaterGroundSurface;
use crate::model::water_body::water_surface::WaterSurface;
use crate::model::water_body::{
    AbstractWaterBoundarySurface, AsAbstractWaterBoundarySurface, AsAbstractWaterBoundarySurfaceMut,
};
use crate::{
    impl_abstract_water_boundary_surface_mut_traits, impl_abstract_water_boundary_surface_traits,
};
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractWaterBoundarySurfaceKind {
    WaterGroundSurface(WaterGroundSurface),
    WaterSurface(WaterSurface),
}

impl AsAbstractWaterBoundarySurface for AbstractWaterBoundarySurfaceKind {
    fn abstract_water_boundary_surface(&self) -> &AbstractWaterBoundarySurface {
        match self {
            AbstractWaterBoundarySurfaceKind::WaterGroundSurface(surface) => {
                surface.abstract_water_boundary_surface()
            }
            AbstractWaterBoundarySurfaceKind::WaterSurface(surface) => {
                surface.abstract_water_boundary_surface()
            }
        }
    }
}

impl AsAbstractWaterBoundarySurfaceMut for AbstractWaterBoundarySurfaceKind {
    fn abstract_water_boundary_surface_mut(&mut self) -> &mut AbstractWaterBoundarySurface {
        match self {
            AbstractWaterBoundarySurfaceKind::WaterGroundSurface(surface) => {
                surface.abstract_water_boundary_surface_mut()
            }
            AbstractWaterBoundarySurfaceKind::WaterSurface(surface) => {
                surface.abstract_water_boundary_surface_mut()
            }
        }
    }
}

impl_abstract_water_boundary_surface_traits!(AbstractWaterBoundarySurfaceKind);
impl_abstract_water_boundary_surface_mut_traits!(AbstractWaterBoundarySurfaceKind);

impl HasFeatureType for AbstractWaterBoundarySurfaceKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::WaterGroundSurface(x) => x.feature_type(),
            Self::WaterSurface(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_water_boundary_surface_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::water_body::AbstractWaterBoundarySurfaceKind {
            fn from(x: $type) -> Self {
                $crate::model::water_body::AbstractWaterBoundarySurfaceKind::$type(x)
            }
        }
        $crate::impl_from_for_thematic_surface_kind!(AbstractWaterBoundarySurfaceKind, $type);
    };
}
impl_from_water_boundary_surface_kind!(WaterGroundSurface);
impl_from_water_boundary_surface_kind!(WaterSurface);

#[macro_export]
macro_rules! impl_try_from_water_boundary_surface_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::water_body::AbstractWaterBoundarySurfaceKind> for $type {
            type Error = ();
            fn try_from(
                x: $crate::model::water_body::AbstractWaterBoundarySurfaceKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::water_body::AbstractWaterBoundarySurfaceKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_thematic_surface_kind!(AbstractWaterBoundarySurfaceKind, $type);
    };
}
impl_try_from_water_boundary_surface_kind!(WaterGroundSurface);
impl_try_from_water_boundary_surface_kind!(WaterSurface);

impl IterFeatures for AbstractWaterBoundarySurfaceKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractWaterBoundarySurfaceKind::WaterGroundSurface(x) => x.iter_features(),
            AbstractWaterBoundarySurfaceKind::WaterSurface(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractWaterBoundarySurfaceKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractWaterBoundarySurfaceKind::WaterGroundSurface(x) => x.for_each_feature_mut(f),
            AbstractWaterBoundarySurfaceKind::WaterSurface(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractWaterBoundarySurfaceKind {
    fn compute_envelope(&self) -> Option<egml::model::geometry::Envelope> {
        match self {
            AbstractWaterBoundarySurfaceKind::WaterGroundSurface(x) => x.compute_envelope(),
            AbstractWaterBoundarySurfaceKind::WaterSurface(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractWaterBoundarySurfaceKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractWaterBoundarySurfaceKind::WaterGroundSurface(x) => x.apply_transform(m),
            AbstractWaterBoundarySurfaceKind::WaterSurface(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractWaterBoundarySurfaceKind::WaterGroundSurface(x) => x.apply_isometry(isometry),
            AbstractWaterBoundarySurfaceKind::WaterSurface(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractWaterBoundarySurfaceKind::WaterGroundSurface(x) => x.apply_translation(vector),
            AbstractWaterBoundarySurfaceKind::WaterSurface(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractWaterBoundarySurfaceKind::WaterGroundSurface(x) => x.apply_rotation(rotation),
            AbstractWaterBoundarySurfaceKind::WaterSurface(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractWaterBoundarySurfaceKind::WaterGroundSurface(x) => x.apply_scale(scale),
            AbstractWaterBoundarySurfaceKind::WaterSurface(x) => x.apply_scale(scale),
        }
    }
}
