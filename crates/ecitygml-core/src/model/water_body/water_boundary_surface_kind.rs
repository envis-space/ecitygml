use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::water_body::water_ground_surface::WaterGroundSurface;
use crate::model::water_body::water_surface::WaterSurface;
use crate::model::water_body::{
    AbstractWaterBoundarySurface, AsAbstractWaterBoundarySurface, AsAbstractWaterBoundarySurfaceMut,
};
use crate::{
    impl_abstract_water_boundary_surface_mut_traits, impl_abstract_water_boundary_surface_traits,
};
use auto_enums::auto_enum;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum WaterBoundarySurfaceKind {
    WaterGroundSurface(WaterGroundSurface),
    WaterSurface(WaterSurface),
}

impl WaterBoundarySurfaceKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            WaterBoundarySurfaceKind::WaterGroundSurface(x) => x.iter_features(),
            WaterBoundarySurfaceKind::WaterSurface(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            WaterBoundarySurfaceKind::WaterGroundSurface(x) => x.for_each_feature_mut(f),
            WaterBoundarySurfaceKind::WaterSurface(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<egml::model::geometry::Envelope> {
        match self {
            WaterBoundarySurfaceKind::WaterGroundSurface(x) => x.compute_envelope(),
            WaterBoundarySurfaceKind::WaterSurface(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            WaterBoundarySurfaceKind::WaterGroundSurface(x) => x.recompute_bounding_shape(),
            WaterBoundarySurfaceKind::WaterSurface(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            WaterBoundarySurfaceKind::WaterGroundSurface(x) => x.apply_transform(m),
            WaterBoundarySurfaceKind::WaterSurface(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractWaterBoundarySurface for WaterBoundarySurfaceKind {
    fn abstract_water_boundary_surface(&self) -> &AbstractWaterBoundarySurface {
        match self {
            WaterBoundarySurfaceKind::WaterGroundSurface(surface) => {
                surface.abstract_water_boundary_surface()
            }
            WaterBoundarySurfaceKind::WaterSurface(surface) => {
                surface.abstract_water_boundary_surface()
            }
        }
    }
}

impl AsAbstractWaterBoundarySurfaceMut for WaterBoundarySurfaceKind {
    fn abstract_water_boundary_surface_mut(&mut self) -> &mut AbstractWaterBoundarySurface {
        match self {
            WaterBoundarySurfaceKind::WaterGroundSurface(surface) => {
                surface.abstract_water_boundary_surface_mut()
            }
            WaterBoundarySurfaceKind::WaterSurface(surface) => {
                surface.abstract_water_boundary_surface_mut()
            }
        }
    }
}

impl_abstract_water_boundary_surface_traits!(WaterBoundarySurfaceKind);
impl_abstract_water_boundary_surface_mut_traits!(WaterBoundarySurfaceKind);

impl HasFeatureType for WaterBoundarySurfaceKind {
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
        impl From<$type> for $crate::model::water_body::WaterBoundarySurfaceKind {
            fn from(x: $type) -> Self {
                $crate::model::water_body::WaterBoundarySurfaceKind::$type(x)
            }
        }
        $crate::impl_from_for_thematic_surface_kind!(WaterBoundarySurfaceKind, $type);
    };
}
impl_from_water_boundary_surface_kind!(WaterGroundSurface);
impl_from_water_boundary_surface_kind!(WaterSurface);

#[macro_export]
macro_rules! impl_try_from_water_boundary_surface_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::water_body::WaterBoundarySurfaceKind> for $type {
            type Error = ();
            fn try_from(
                x: $crate::model::water_body::WaterBoundarySurfaceKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::water_body::WaterBoundarySurfaceKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_thematic_surface_kind!(WaterBoundarySurfaceKind, $type);
    };
}
impl_try_from_water_boundary_surface_kind!(WaterGroundSurface);
impl_try_from_water_boundary_surface_kind!(WaterSurface);
