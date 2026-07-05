use crate::impl_try_from_thematic_surface_kind_ref_for_enum;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::water_body::{
    AbstractWaterBoundarySurface, AsAbstractWaterBoundarySurface, WaterBoundarySurfaceKind,
    WaterGroundSurface, WaterSurface,
};

#[derive(Debug, Clone, Copy)]
pub enum WaterBoundarySurfaceKindRef<'a> {
    WaterGroundSurface(&'a WaterGroundSurface),
    WaterSurface(&'a WaterSurface),
}

impl<'a> From<&'a WaterBoundarySurfaceKind> for WaterBoundarySurfaceKindRef<'a> {
    fn from(item: &'a WaterBoundarySurfaceKind) -> Self {
        match item {
            WaterBoundarySurfaceKind::WaterGroundSurface(x) => Self::WaterGroundSurface(x),
            WaterBoundarySurfaceKind::WaterSurface(x) => Self::WaterSurface(x),
        }
    }
}

impl<'a> AsAbstractWaterBoundarySurface for WaterBoundarySurfaceKindRef<'a> {
    fn abstract_water_boundary_surface(&self) -> &AbstractWaterBoundarySurface {
        match self {
            Self::WaterGroundSurface(x) => x.abstract_water_boundary_surface(),
            Self::WaterSurface(x) => x.abstract_water_boundary_surface(),
        }
    }
}
crate::impl_abstract_water_boundary_surface_traits!(WaterBoundarySurfaceKindRef<'_>);

impl<'a> HasFeatureType for WaterBoundarySurfaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::WaterGroundSurface(_) => FeatureType::WaterGroundSurface,
            Self::WaterSurface(_) => FeatureType::WaterSurface,
        }
    }
}

#[macro_export]
macro_rules! impl_from_water_boundary_surface_kind_ref {
    ($type:ident) => {
        impl<'a> From<&'a $type> for $crate::model::water_body::WaterBoundarySurfaceKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::water_body::WaterBoundarySurfaceKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_thematic_surface_kind_ref!(WaterBoundarySurfaceKind, $type);
    };
}
impl_from_water_boundary_surface_kind_ref!(WaterGroundSurface);
impl_from_water_boundary_surface_kind_ref!(WaterSurface);

#[macro_export]
macro_rules! impl_try_from_water_boundary_surface_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::water_body::WaterBoundarySurfaceKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::water_body::WaterBoundarySurfaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::water_body::WaterBoundarySurfaceKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_thematic_surface_kind_ref!(WaterBoundarySurfaceKind, $type);
    };
}
impl_try_from_water_boundary_surface_kind_ref!(WaterGroundSurface);
impl_try_from_water_boundary_surface_kind_ref!(WaterSurface);
impl_try_from_thematic_surface_kind_ref_for_enum!(
    WaterBoundarySurfaceKind,
    WaterBoundarySurfaceKindRef
);
