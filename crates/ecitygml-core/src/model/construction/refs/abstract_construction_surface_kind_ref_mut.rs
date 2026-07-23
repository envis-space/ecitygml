use crate::impl_try_from_thematic_surface_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::{
    AbstractConstructionSurface, AbstractConstructionSurfaceKind, AsAbstractConstructionSurface,
    AsAbstractConstructionSurfaceMut, CeilingSurface, FloorSurface, GroundSurface,
    InteriorWallSurface, OuterCeilingSurface, OuterFloorSurface, RoofSurface, WallSurface,
};
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractConstructionSurfaceKindRefMut<'a> {
    CeilingSurface(&'a mut CeilingSurface),
    FloorSurface(&'a mut FloorSurface),
    GroundSurface(&'a mut GroundSurface),
    InteriorWallSurface(&'a mut InteriorWallSurface),
    OuterCeilingSurface(&'a mut OuterCeilingSurface),
    OuterFloorSurface(&'a mut OuterFloorSurface),
    RoofSurface(&'a mut RoofSurface),
    WallSurface(&'a mut WallSurface),
}

impl<'a> From<&'a mut AbstractConstructionSurfaceKind>
    for AbstractConstructionSurfaceKindRefMut<'a>
{
    fn from(item: &'a mut AbstractConstructionSurfaceKind) -> Self {
        match item {
            AbstractConstructionSurfaceKind::CeilingSurface(x) => Self::CeilingSurface(x),
            AbstractConstructionSurfaceKind::FloorSurface(x) => Self::FloorSurface(x),
            AbstractConstructionSurfaceKind::GroundSurface(x) => Self::GroundSurface(x),
            AbstractConstructionSurfaceKind::InteriorWallSurface(x) => Self::InteriorWallSurface(x),
            AbstractConstructionSurfaceKind::OuterCeilingSurface(x) => Self::OuterCeilingSurface(x),
            AbstractConstructionSurfaceKind::OuterFloorSurface(x) => Self::OuterFloorSurface(x),
            AbstractConstructionSurfaceKind::RoofSurface(x) => Self::RoofSurface(x),
            AbstractConstructionSurfaceKind::WallSurface(x) => Self::WallSurface(x),
        }
    }
}

impl<'a> AsAbstractConstructionSurface for AbstractConstructionSurfaceKindRefMut<'a> {
    fn abstract_construction_surface(&self) -> &AbstractConstructionSurface {
        match self {
            Self::CeilingSurface(x) => x.abstract_construction_surface(),
            Self::FloorSurface(x) => x.abstract_construction_surface(),
            Self::GroundSurface(x) => x.abstract_construction_surface(),
            Self::InteriorWallSurface(x) => x.abstract_construction_surface(),
            Self::OuterCeilingSurface(x) => x.abstract_construction_surface(),
            Self::OuterFloorSurface(x) => x.abstract_construction_surface(),
            Self::RoofSurface(x) => x.abstract_construction_surface(),
            Self::WallSurface(x) => x.abstract_construction_surface(),
        }
    }
}

impl<'a> AsAbstractConstructionSurfaceMut for AbstractConstructionSurfaceKindRefMut<'a> {
    fn abstract_construction_surface_mut(&mut self) -> &mut AbstractConstructionSurface {
        match self {
            Self::CeilingSurface(x) => x.abstract_construction_surface_mut(),
            Self::FloorSurface(x) => x.abstract_construction_surface_mut(),
            Self::GroundSurface(x) => x.abstract_construction_surface_mut(),
            Self::InteriorWallSurface(x) => x.abstract_construction_surface_mut(),
            Self::OuterCeilingSurface(x) => x.abstract_construction_surface_mut(),
            Self::OuterFloorSurface(x) => x.abstract_construction_surface_mut(),
            Self::RoofSurface(x) => x.abstract_construction_surface_mut(),
            Self::WallSurface(x) => x.abstract_construction_surface_mut(),
        }
    }
}
crate::impl_abstract_construction_surface_traits!(AbstractConstructionSurfaceKindRefMut<'_>);
crate::impl_abstract_construction_surface_mut_traits!(AbstractConstructionSurfaceKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractConstructionSurfaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::CeilingSurface(x) => x.feature_type(),
            Self::FloorSurface(x) => x.feature_type(),
            Self::GroundSurface(x) => x.feature_type(),
            Self::InteriorWallSurface(x) => x.feature_type(),
            Self::OuterCeilingSurface(x) => x.feature_type(),
            Self::OuterFloorSurface(x) => x.feature_type(),
            Self::RoofSurface(x) => x.feature_type(),
            Self::WallSurface(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_construction_surface_kind_ref_mut {
    ($type:ident) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::construction::refs::AbstractConstructionSurfaceKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::construction::refs::AbstractConstructionSurfaceKindRefMut::$type(
                    x.into(),
                )
            }
        }
        $crate::impl_from_for_thematic_surface_kind_ref_mut!(
            AbstractConstructionSurfaceKind,
            $type
        );
    };
}
impl_from_construction_surface_kind_ref_mut!(CeilingSurface);
impl_from_construction_surface_kind_ref_mut!(FloorSurface);
impl_from_construction_surface_kind_ref_mut!(GroundSurface);
impl_from_construction_surface_kind_ref_mut!(InteriorWallSurface);
impl_from_construction_surface_kind_ref_mut!(OuterCeilingSurface);
impl_from_construction_surface_kind_ref_mut!(OuterFloorSurface);
impl_from_construction_surface_kind_ref_mut!(RoofSurface);
impl_from_construction_surface_kind_ref_mut!(WallSurface);

#[macro_export]
macro_rules! impl_try_from_construction_surface_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::construction::refs::AbstractConstructionSurfaceKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::refs::AbstractConstructionSurfaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::refs::AbstractConstructionSurfaceKindRefMut::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_thematic_surface_kind_ref_mut!(AbstractConstructionSurfaceKind, $type);
    };
}
impl_try_from_construction_surface_kind_ref_mut!(CeilingSurface);
impl_try_from_construction_surface_kind_ref_mut!(FloorSurface);
impl_try_from_construction_surface_kind_ref_mut!(GroundSurface);
impl_try_from_construction_surface_kind_ref_mut!(InteriorWallSurface);
impl_try_from_construction_surface_kind_ref_mut!(OuterCeilingSurface);
impl_try_from_construction_surface_kind_ref_mut!(OuterFloorSurface);
impl_try_from_construction_surface_kind_ref_mut!(RoofSurface);
impl_try_from_construction_surface_kind_ref_mut!(WallSurface);
impl_try_from_thematic_surface_kind_ref_mut_for_enum!(
    AbstractConstructionSurfaceKind,
    AbstractConstructionSurfaceKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractConstructionSurfaceKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::CeilingSurface(x) => x.recompute_bounding_shape(),
            Self::FloorSurface(x) => x.recompute_bounding_shape(),
            Self::GroundSurface(x) => x.recompute_bounding_shape(),
            Self::InteriorWallSurface(x) => x.recompute_bounding_shape(),
            Self::OuterCeilingSurface(x) => x.recompute_bounding_shape(),
            Self::OuterFloorSurface(x) => x.recompute_bounding_shape(),
            Self::RoofSurface(x) => x.recompute_bounding_shape(),
            Self::WallSurface(x) => x.recompute_bounding_shape(),
        }
    }
}
