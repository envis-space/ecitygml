use crate::impl_try_from_thematic_surface_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::{
    AbstractConstructionSurface, AsAbstractConstructionSurface, CeilingSurface,
    ConstructionSurfaceKind, FloorSurface, GroundSurface, InteriorWallSurface, OuterCeilingSurface,
    OuterFloorSurface, RoofSurface, WallSurface,
};

#[derive(Debug, Clone, Copy)]
pub enum ConstructionSurfaceKindRef<'a> {
    CeilingSurface(&'a CeilingSurface),
    FloorSurface(&'a FloorSurface),
    GroundSurface(&'a GroundSurface),
    InteriorWallSurface(&'a InteriorWallSurface),
    OuterCeilingSurface(&'a OuterCeilingSurface),
    OuterFloorSurface(&'a OuterFloorSurface),
    RoofSurface(&'a RoofSurface),
    WallSurface(&'a WallSurface),
}

impl<'a> From<&'a ConstructionSurfaceKind> for ConstructionSurfaceKindRef<'a> {
    fn from(item: &'a ConstructionSurfaceKind) -> Self {
        match item {
            ConstructionSurfaceKind::CeilingSurface(x) => Self::CeilingSurface(x),
            ConstructionSurfaceKind::FloorSurface(x) => Self::FloorSurface(x),
            ConstructionSurfaceKind::GroundSurface(x) => Self::GroundSurface(x),
            ConstructionSurfaceKind::InteriorWallSurface(x) => Self::InteriorWallSurface(x),
            ConstructionSurfaceKind::OuterCeilingSurface(x) => Self::OuterCeilingSurface(x),
            ConstructionSurfaceKind::OuterFloorSurface(x) => Self::OuterFloorSurface(x),
            ConstructionSurfaceKind::RoofSurface(x) => Self::RoofSurface(x),
            ConstructionSurfaceKind::WallSurface(x) => Self::WallSurface(x),
        }
    }
}

impl<'a> AsAbstractConstructionSurface for ConstructionSurfaceKindRef<'a> {
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
crate::impl_abstract_construction_surface_traits!(ConstructionSurfaceKindRef<'_>);

impl<'a> HasFeatureType for ConstructionSurfaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::CeilingSurface(_) => FeatureType::CeilingSurface,
            Self::FloorSurface(_) => FeatureType::FloorSurface,
            Self::GroundSurface(_) => FeatureType::GroundSurface,
            Self::InteriorWallSurface(_) => FeatureType::InteriorWallSurface,
            Self::OuterCeilingSurface(_) => FeatureType::OuterCeilingSurface,
            Self::OuterFloorSurface(_) => FeatureType::OuterFloorSurface,
            Self::RoofSurface(_) => FeatureType::RoofSurface,
            Self::WallSurface(_) => FeatureType::WallSurface,
        }
    }
}

#[macro_export]
macro_rules! impl_from_construction_surface_kind_ref {
    ($type:ident) => {
        impl<'a> From<&'a $type>
            for $crate::model::construction::refs::ConstructionSurfaceKindRef<'a>
        {
            fn from(x: &'a $type) -> Self {
                $crate::model::construction::refs::ConstructionSurfaceKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_thematic_surface_kind_ref!(ConstructionSurfaceKind, $type);
    };
}
impl_from_construction_surface_kind_ref!(CeilingSurface);
impl_from_construction_surface_kind_ref!(FloorSurface);
impl_from_construction_surface_kind_ref!(GroundSurface);
impl_from_construction_surface_kind_ref!(InteriorWallSurface);
impl_from_construction_surface_kind_ref!(OuterCeilingSurface);
impl_from_construction_surface_kind_ref!(OuterFloorSurface);
impl_from_construction_surface_kind_ref!(RoofSurface);
impl_from_construction_surface_kind_ref!(WallSurface);

#[macro_export]
macro_rules! impl_try_from_construction_surface_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::construction::refs::ConstructionSurfaceKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::refs::ConstructionSurfaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::refs::ConstructionSurfaceKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_thematic_surface_kind_ref!(ConstructionSurfaceKind, $type);
    };
}
impl_try_from_construction_surface_kind_ref!(CeilingSurface);
impl_try_from_construction_surface_kind_ref!(FloorSurface);
impl_try_from_construction_surface_kind_ref!(GroundSurface);
impl_try_from_construction_surface_kind_ref!(InteriorWallSurface);
impl_try_from_construction_surface_kind_ref!(OuterCeilingSurface);
impl_try_from_construction_surface_kind_ref!(OuterFloorSurface);
impl_try_from_construction_surface_kind_ref!(RoofSurface);
impl_try_from_construction_surface_kind_ref!(WallSurface);
impl_try_from_thematic_surface_kind_ref_for_enum!(
    ConstructionSurfaceKind,
    ConstructionSurfaceKindRef
);
