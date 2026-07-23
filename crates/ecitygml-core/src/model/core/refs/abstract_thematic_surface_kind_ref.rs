use crate::impl_try_from_space_boundary_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::refs::{
    AbstractConstructionSurfaceKindRef, AbstractFillingSurfaceKindRef,
};
use crate::model::construction::{AbstractConstructionSurfaceKind, AbstractFillingSurfaceKind};
use crate::model::core::{
    AbstractThematicSurface, AbstractThematicSurfaceKind, AsAbstractThematicSurface, ClosureSurface,
};
use crate::model::generics::GenericThematicSurface;
use crate::model::land_use::LandUse;
use crate::model::transportation::{AuxiliaryTrafficArea, HoleSurface, Marking, TrafficArea};
use crate::model::water_body::{
    AbstractWaterBoundarySurfaceKind, AbstractWaterBoundarySurfaceKindRef,
};

#[derive(Debug, Clone, Copy)]
pub enum AbstractThematicSurfaceKindRef<'a> {
    AuxiliaryTrafficArea(&'a AuxiliaryTrafficArea),
    ClosureSurface(&'a ClosureSurface),
    AbstractConstructionSurfaceKind(AbstractConstructionSurfaceKindRef<'a>),
    AbstractFillingSurfaceKind(AbstractFillingSurfaceKindRef<'a>),
    GenericThematicSurface(&'a GenericThematicSurface),
    HoleSurface(&'a HoleSurface),
    LandUse(&'a LandUse),
    Marking(&'a Marking),
    TrafficArea(&'a TrafficArea),
    AbstractWaterBoundarySurfaceKind(AbstractWaterBoundarySurfaceKindRef<'a>),
}

impl<'a> From<&'a AbstractThematicSurfaceKind> for AbstractThematicSurfaceKindRef<'a> {
    fn from(item: &'a AbstractThematicSurfaceKind) -> Self {
        match item {
            AbstractThematicSurfaceKind::AuxiliaryTrafficArea(x) => Self::AuxiliaryTrafficArea(x),
            AbstractThematicSurfaceKind::ClosureSurface(x) => Self::ClosureSurface(x),
            AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(x) => {
                Self::AbstractConstructionSurfaceKind(x.into())
            }
            AbstractThematicSurfaceKind::AbstractFillingSurfaceKind(x) => {
                Self::AbstractFillingSurfaceKind(x.into())
            }
            AbstractThematicSurfaceKind::GenericThematicSurface(x) => {
                Self::GenericThematicSurface(x)
            }
            AbstractThematicSurfaceKind::HoleSurface(x) => Self::HoleSurface(x),
            AbstractThematicSurfaceKind::LandUse(x) => Self::LandUse(x),
            AbstractThematicSurfaceKind::Marking(x) => Self::Marking(x),
            AbstractThematicSurfaceKind::TrafficArea(x) => Self::TrafficArea(x),
            AbstractThematicSurfaceKind::AbstractWaterBoundarySurfaceKind(x) => {
                Self::AbstractWaterBoundarySurfaceKind(x.into())
            }
        }
    }
}

impl<'a> AsAbstractThematicSurface for AbstractThematicSurfaceKindRef<'a> {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.abstract_thematic_surface(),
            Self::ClosureSurface(x) => x.abstract_thematic_surface(),
            Self::AbstractConstructionSurfaceKind(x) => x.abstract_thematic_surface(),
            Self::AbstractFillingSurfaceKind(x) => x.abstract_thematic_surface(),
            Self::GenericThematicSurface(x) => x.abstract_thematic_surface(),
            Self::HoleSurface(x) => x.abstract_thematic_surface(),
            Self::LandUse(x) => x.abstract_thematic_surface(),
            Self::Marking(x) => x.abstract_thematic_surface(),
            Self::TrafficArea(x) => x.abstract_thematic_surface(),
            Self::AbstractWaterBoundarySurfaceKind(x) => x.abstract_thematic_surface(),
        }
    }
}
crate::impl_abstract_thematic_surface_traits!(AbstractThematicSurfaceKindRef<'_>);

impl<'a> HasFeatureType for AbstractThematicSurfaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::AuxiliaryTrafficArea(_) => FeatureType::AuxiliaryTrafficArea,
            Self::ClosureSurface(_) => FeatureType::ClosureSurface,
            Self::AbstractConstructionSurfaceKind(x) => x.feature_type(),
            Self::AbstractFillingSurfaceKind(x) => x.feature_type(),
            Self::GenericThematicSurface(_) => FeatureType::GenericThematicSurface,
            Self::HoleSurface(_) => FeatureType::HoleSurface,
            Self::LandUse(_) => FeatureType::LandUse,
            Self::Marking(_) => FeatureType::Marking,
            Self::TrafficArea(_) => FeatureType::TrafficArea,
            Self::AbstractWaterBoundarySurfaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_thematic_surface_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::AbstractThematicSurfaceKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::AbstractThematicSurfaceKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_space_boundary_kind_ref!(AbstractThematicSurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_thematic_surface_kind_ref!($variant, $variant);
    };
}
impl_from_for_thematic_surface_kind_ref!(AuxiliaryTrafficArea);
impl_from_for_thematic_surface_kind_ref!(ClosureSurface);
impl_from_for_thematic_surface_kind_ref!(AbstractConstructionSurfaceKind);
impl_from_for_thematic_surface_kind_ref!(AbstractFillingSurfaceKind);
impl_from_for_thematic_surface_kind_ref!(GenericThematicSurface);
impl_from_for_thematic_surface_kind_ref!(HoleSurface);
impl_from_for_thematic_surface_kind_ref!(LandUse);
impl_from_for_thematic_surface_kind_ref!(Marking);
impl_from_for_thematic_surface_kind_ref!(TrafficArea);
impl_from_for_thematic_surface_kind_ref!(AbstractWaterBoundarySurfaceKind);

#[macro_export]
macro_rules! impl_try_from_for_thematic_surface_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractThematicSurfaceKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractThematicSurfaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractThematicSurfaceKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_boundary_kind_ref!(AbstractThematicSurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_thematic_surface_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_thematic_surface_kind_ref!(AuxiliaryTrafficArea);
impl_try_from_for_thematic_surface_kind_ref!(ClosureSurface);
impl_try_from_for_thematic_surface_kind_ref!(GenericThematicSurface);
impl_try_from_for_thematic_surface_kind_ref!(HoleSurface);
impl_try_from_for_thematic_surface_kind_ref!(LandUse);
impl_try_from_for_thematic_surface_kind_ref!(Marking);
impl_try_from_for_thematic_surface_kind_ref!(TrafficArea);

#[macro_export]
macro_rules! impl_try_from_thematic_surface_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractThematicSurfaceKindRef<'a>>
            for $EnumRef<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractThematicSurfaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractThematicSurfaceKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_space_boundary_kind_ref_for_enum!(
            AbstractThematicSurfaceKind,
            $EnumRef
        );
    };
}
impl_try_from_space_boundary_kind_ref_for_enum!(
    AbstractThematicSurfaceKind,
    AbstractThematicSurfaceKindRef
);
