use crate::impl_try_from_space_boundary_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::refs::{ConstructionSurfaceKindRef, FillingSurfaceKindRef};
use crate::model::construction::{ConstructionSurfaceKind, FillingSurfaceKind};
use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, ClosureSurface, ThematicSurfaceKind,
};
use crate::model::generics::GenericThematicSurface;
use crate::model::land_use::LandUse;
use crate::model::transportation::{AuxiliaryTrafficArea, Marking, TrafficArea};
use crate::model::water_body::{WaterBoundarySurfaceKind, WaterBoundarySurfaceKindRef};

#[derive(Debug, Clone, Copy)]
pub enum ThematicSurfaceKindRef<'a> {
    AuxiliaryTrafficArea(&'a AuxiliaryTrafficArea),
    ClosureSurface(&'a ClosureSurface),
    ConstructionSurfaceKind(ConstructionSurfaceKindRef<'a>),
    FillingSurfaceKind(FillingSurfaceKindRef<'a>),
    GenericThematicSurface(&'a GenericThematicSurface),
    LandUse(&'a LandUse),
    Marking(&'a Marking),
    TrafficArea(&'a TrafficArea),
    WaterBoundarySurfaceKind(WaterBoundarySurfaceKindRef<'a>),
}

impl<'a> From<&'a ThematicSurfaceKind> for ThematicSurfaceKindRef<'a> {
    fn from(item: &'a ThematicSurfaceKind) -> Self {
        match item {
            ThematicSurfaceKind::AuxiliaryTrafficArea(x) => Self::AuxiliaryTrafficArea(x),
            ThematicSurfaceKind::ClosureSurface(x) => Self::ClosureSurface(x),
            ThematicSurfaceKind::ConstructionSurfaceKind(x) => {
                Self::ConstructionSurfaceKind(x.into())
            }
            ThematicSurfaceKind::FillingSurfaceKind(x) => Self::FillingSurfaceKind(x.into()),
            ThematicSurfaceKind::GenericThematicSurface(x) => Self::GenericThematicSurface(x),
            ThematicSurfaceKind::LandUse(x) => Self::LandUse(x),
            ThematicSurfaceKind::Marking(x) => Self::Marking(x),
            ThematicSurfaceKind::TrafficArea(x) => Self::TrafficArea(x),
            ThematicSurfaceKind::WaterBoundarySurfaceKind(x) => {
                Self::WaterBoundarySurfaceKind(x.into())
            }
        }
    }
}

impl<'a> AsAbstractThematicSurface for ThematicSurfaceKindRef<'a> {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.abstract_thematic_surface(),
            Self::ClosureSurface(x) => x.abstract_thematic_surface(),
            Self::ConstructionSurfaceKind(x) => x.abstract_thematic_surface(),
            Self::FillingSurfaceKind(x) => x.abstract_thematic_surface(),
            Self::GenericThematicSurface(x) => x.abstract_thematic_surface(),
            Self::LandUse(x) => x.abstract_thematic_surface(),
            Self::Marking(x) => x.abstract_thematic_surface(),
            Self::TrafficArea(x) => x.abstract_thematic_surface(),
            Self::WaterBoundarySurfaceKind(x) => x.abstract_thematic_surface(),
        }
    }
}
crate::impl_abstract_thematic_surface_traits!(ThematicSurfaceKindRef<'_>);

impl<'a> HasFeatureType for ThematicSurfaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::AuxiliaryTrafficArea(_) => FeatureType::AuxiliaryTrafficArea,
            Self::ClosureSurface(_) => FeatureType::ClosureSurface,
            Self::ConstructionSurfaceKind(x) => x.feature_type(),
            Self::FillingSurfaceKind(x) => x.feature_type(),
            Self::GenericThematicSurface(_) => FeatureType::GenericThematicSurface,
            Self::LandUse(_) => FeatureType::LandUse,
            Self::Marking(_) => FeatureType::Marking,
            Self::TrafficArea(_) => FeatureType::TrafficArea,
            Self::WaterBoundarySurfaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_thematic_surface_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::ThematicSurfaceKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::ThematicSurfaceKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_boundary_kind_ref!(ThematicSurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_thematic_surface_kind_ref!($variant, $variant);
    };
}
impl_from_for_thematic_surface_kind_ref!(AuxiliaryTrafficArea);
impl_from_for_thematic_surface_kind_ref!(ClosureSurface);
impl_from_for_thematic_surface_kind_ref!(ConstructionSurfaceKind);
impl_from_for_thematic_surface_kind_ref!(FillingSurfaceKind);
impl_from_for_thematic_surface_kind_ref!(GenericThematicSurface);
impl_from_for_thematic_surface_kind_ref!(LandUse);
impl_from_for_thematic_surface_kind_ref!(Marking);
impl_from_for_thematic_surface_kind_ref!(TrafficArea);
impl_from_for_thematic_surface_kind_ref!(WaterBoundarySurfaceKind);

#[macro_export]
macro_rules! impl_try_from_for_thematic_surface_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::ThematicSurfaceKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::ThematicSurfaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::ThematicSurfaceKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_boundary_kind_ref!(ThematicSurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_thematic_surface_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_thematic_surface_kind_ref!(AuxiliaryTrafficArea);
impl_try_from_for_thematic_surface_kind_ref!(ClosureSurface);
impl_try_from_for_thematic_surface_kind_ref!(GenericThematicSurface);
impl_try_from_for_thematic_surface_kind_ref!(LandUse);
impl_try_from_for_thematic_surface_kind_ref!(Marking);
impl_try_from_for_thematic_surface_kind_ref!(TrafficArea);

#[macro_export]
macro_rules! impl_try_from_thematic_surface_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::ThematicSurfaceKindRef<'a>> for $EnumRef<'a> {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::ThematicSurfaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::ThematicSurfaceKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_space_boundary_kind_ref_for_enum!(ThematicSurfaceKind, $EnumRef);
    };
}
impl_try_from_space_boundary_kind_ref_for_enum!(ThematicSurfaceKind, ThematicSurfaceKindRef);
