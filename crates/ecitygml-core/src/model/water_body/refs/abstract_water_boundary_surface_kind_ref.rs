use crate::impl_try_from_thematic_surface_kind_ref_for_enum;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::water_body::{
    AbstractWaterBoundarySurface, AbstractWaterBoundarySurfaceKind, AsAbstractWaterBoundarySurface,
    WaterGroundSurface, WaterSurface,
};

#[derive(Debug, Clone, Copy)]
pub enum AbstractWaterBoundarySurfaceKindRef<'a> {
    WaterGroundSurface(&'a WaterGroundSurface),
    WaterSurface(&'a WaterSurface),
}

impl<'a> From<&'a AbstractWaterBoundarySurfaceKind> for AbstractWaterBoundarySurfaceKindRef<'a> {
    fn from(item: &'a AbstractWaterBoundarySurfaceKind) -> Self {
        match item {
            AbstractWaterBoundarySurfaceKind::WaterGroundSurface(x) => Self::WaterGroundSurface(x),
            AbstractWaterBoundarySurfaceKind::WaterSurface(x) => Self::WaterSurface(x),
        }
    }
}

impl<'a> AsAbstractWaterBoundarySurface for AbstractWaterBoundarySurfaceKindRef<'a> {
    fn abstract_water_boundary_surface(&self) -> &AbstractWaterBoundarySurface {
        match self {
            Self::WaterGroundSurface(x) => x.abstract_water_boundary_surface(),
            Self::WaterSurface(x) => x.abstract_water_boundary_surface(),
        }
    }
}
crate::impl_abstract_water_boundary_surface_traits!(AbstractWaterBoundarySurfaceKindRef<'_>);

impl<'a> HasFeatureType for AbstractWaterBoundarySurfaceKindRef<'a> {
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
        impl<'a> From<&'a $type>
            for $crate::model::water_body::AbstractWaterBoundarySurfaceKindRef<'a>
        {
            fn from(x: &'a $type) -> Self {
                $crate::model::water_body::AbstractWaterBoundarySurfaceKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_thematic_surface_kind_ref!(AbstractWaterBoundarySurfaceKind, $type);
    };
}
impl_from_water_boundary_surface_kind_ref!(WaterGroundSurface);
impl_from_water_boundary_surface_kind_ref!(WaterSurface);

#[macro_export]
macro_rules! impl_try_from_water_boundary_surface_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::water_body::AbstractWaterBoundarySurfaceKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::water_body::AbstractWaterBoundarySurfaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::water_body::AbstractWaterBoundarySurfaceKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_thematic_surface_kind_ref!(
            AbstractWaterBoundarySurfaceKind,
            $type
        );
    };
}
impl_try_from_water_boundary_surface_kind_ref!(WaterGroundSurface);
impl_try_from_water_boundary_surface_kind_ref!(WaterSurface);
impl_try_from_thematic_surface_kind_ref_for_enum!(
    AbstractWaterBoundarySurfaceKind,
    AbstractWaterBoundarySurfaceKindRef
);
