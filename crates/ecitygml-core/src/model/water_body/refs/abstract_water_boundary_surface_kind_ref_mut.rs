use crate::impl_try_from_thematic_surface_kind_ref_mut_for_enum;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::water_body::{
    AbstractWaterBoundarySurface, AbstractWaterBoundarySurfaceKind, AsAbstractWaterBoundarySurface,
    AsAbstractWaterBoundarySurfaceMut, WaterGroundSurface, WaterSurface,
};
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractWaterBoundarySurfaceKindRefMut<'a> {
    WaterGroundSurface(&'a mut WaterGroundSurface),
    WaterSurface(&'a mut WaterSurface),
}

impl<'a> From<&'a mut AbstractWaterBoundarySurfaceKind>
    for AbstractWaterBoundarySurfaceKindRefMut<'a>
{
    fn from(item: &'a mut AbstractWaterBoundarySurfaceKind) -> Self {
        match item {
            AbstractWaterBoundarySurfaceKind::WaterGroundSurface(x) => Self::WaterGroundSurface(x),
            AbstractWaterBoundarySurfaceKind::WaterSurface(x) => Self::WaterSurface(x),
        }
    }
}

impl<'a> AsAbstractWaterBoundarySurface for AbstractWaterBoundarySurfaceKindRefMut<'a> {
    fn abstract_water_boundary_surface(&self) -> &AbstractWaterBoundarySurface {
        match self {
            Self::WaterGroundSurface(x) => x.abstract_water_boundary_surface(),
            Self::WaterSurface(x) => x.abstract_water_boundary_surface(),
        }
    }
}

impl<'a> AsAbstractWaterBoundarySurfaceMut for AbstractWaterBoundarySurfaceKindRefMut<'a> {
    fn abstract_water_boundary_surface_mut(&mut self) -> &mut AbstractWaterBoundarySurface {
        match self {
            Self::WaterGroundSurface(x) => x.abstract_water_boundary_surface_mut(),
            Self::WaterSurface(x) => x.abstract_water_boundary_surface_mut(),
        }
    }
}
crate::impl_abstract_water_boundary_surface_traits!(AbstractWaterBoundarySurfaceKindRefMut<'_>);
crate::impl_abstract_water_boundary_surface_mut_traits!(AbstractWaterBoundarySurfaceKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractWaterBoundarySurfaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::WaterGroundSurface(x) => x.feature_type(),
            Self::WaterSurface(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_water_boundary_surface_kind_ref_mut {
    ($type:ident) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::water_body::AbstractWaterBoundarySurfaceKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::water_body::AbstractWaterBoundarySurfaceKindRefMut::$type(x.into())
            }
        }
        $crate::impl_from_for_thematic_surface_kind_ref_mut!(
            AbstractWaterBoundarySurfaceKind,
            $type
        );
    };
}
impl_from_water_boundary_surface_kind_ref_mut!(WaterGroundSurface);
impl_from_water_boundary_surface_kind_ref_mut!(WaterSurface);

#[macro_export]
macro_rules! impl_try_from_water_boundary_surface_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::water_body::AbstractWaterBoundarySurfaceKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::water_body::AbstractWaterBoundarySurfaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::water_body::AbstractWaterBoundarySurfaceKindRefMut::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_thematic_surface_kind_ref_mut!(
            AbstractWaterBoundarySurfaceKind,
            $type
        );
    };
}
impl_try_from_water_boundary_surface_kind_ref_mut!(WaterGroundSurface);
impl_try_from_water_boundary_surface_kind_ref_mut!(WaterSurface);
impl_try_from_thematic_surface_kind_ref_mut_for_enum!(
    AbstractWaterBoundarySurfaceKind,
    AbstractWaterBoundarySurfaceKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractWaterBoundarySurfaceKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::WaterGroundSurface(x) => x.recompute_bounding_shape(),
            Self::WaterSurface(x) => x.recompute_bounding_shape(),
        }
    }
}
