use crate::impl_try_from_thematic_surface_kind_ref_mut_for_enum;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::water_body::{
    AbstractWaterBoundarySurface, AsAbstractWaterBoundarySurface,
    AsAbstractWaterBoundarySurfaceMut, WaterBoundarySurfaceKind, WaterGroundSurface, WaterSurface,
};

#[derive(Debug)]
pub enum WaterBoundarySurfaceKindRefMut<'a> {
    WaterGroundSurface(&'a mut WaterGroundSurface),
    WaterSurface(&'a mut WaterSurface),
}

impl<'a> From<&'a mut WaterBoundarySurfaceKind> for WaterBoundarySurfaceKindRefMut<'a> {
    fn from(item: &'a mut WaterBoundarySurfaceKind) -> Self {
        match item {
            WaterBoundarySurfaceKind::WaterGroundSurface(x) => Self::WaterGroundSurface(x),
            WaterBoundarySurfaceKind::WaterSurface(x) => Self::WaterSurface(x),
        }
    }
}

impl<'a> AsAbstractWaterBoundarySurface for WaterBoundarySurfaceKindRefMut<'a> {
    fn abstract_water_boundary_surface(&self) -> &AbstractWaterBoundarySurface {
        match self {
            Self::WaterGroundSurface(x) => x.abstract_water_boundary_surface(),
            Self::WaterSurface(x) => x.abstract_water_boundary_surface(),
        }
    }
}

impl<'a> AsAbstractWaterBoundarySurfaceMut for WaterBoundarySurfaceKindRefMut<'a> {
    fn abstract_water_boundary_surface_mut(&mut self) -> &mut AbstractWaterBoundarySurface {
        match self {
            Self::WaterGroundSurface(x) => x.abstract_water_boundary_surface_mut(),
            Self::WaterSurface(x) => x.abstract_water_boundary_surface_mut(),
        }
    }
}
crate::impl_abstract_water_boundary_surface_traits!(WaterBoundarySurfaceKindRefMut<'_>);
crate::impl_abstract_water_boundary_surface_mut_traits!(WaterBoundarySurfaceKindRefMut<'_>);

impl<'a> WaterBoundarySurfaceKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::WaterGroundSurface(x) => x.recompute_bounding_shape(),
            Self::WaterSurface(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for WaterBoundarySurfaceKindRefMut<'a> {
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
            for $crate::model::water_body::WaterBoundarySurfaceKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::water_body::WaterBoundarySurfaceKindRefMut::$type(x.into())
            }
        }
        $crate::impl_from_for_thematic_surface_kind_ref_mut!(WaterBoundarySurfaceKind, $type);
    };
}
impl_from_water_boundary_surface_kind_ref_mut!(WaterGroundSurface);
impl_from_water_boundary_surface_kind_ref_mut!(WaterSurface);

#[macro_export]
macro_rules! impl_try_from_water_boundary_surface_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::water_body::WaterBoundarySurfaceKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::water_body::WaterBoundarySurfaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::water_body::WaterBoundarySurfaceKindRefMut::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_thematic_surface_kind_ref_mut!(WaterBoundarySurfaceKind, $type);
    };
}
impl_try_from_water_boundary_surface_kind_ref_mut!(WaterGroundSurface);
impl_try_from_water_boundary_surface_kind_ref_mut!(WaterSurface);
impl_try_from_thematic_surface_kind_ref_mut_for_enum!(
    WaterBoundarySurfaceKind,
    WaterBoundarySurfaceKindRefMut
);
