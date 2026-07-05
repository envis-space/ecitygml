use crate::impl_try_from_physical_space_kind_ref_for_enum;
use crate::model::building::BuildingRoom;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::{AbstractUnoccupiedSpace, AsAbstractUnoccupiedSpace, UnoccupiedSpaceKind};
use crate::model::transportation::refs::TransportationSpaceKindRef;
use crate::model::transportation::{
    AuxiliaryTrafficSpace, ClearanceSpace, TrafficSpace, TransportationSpaceKind,
};

#[derive(Debug, Clone, Copy)]
pub enum UnoccupiedSpaceKindRef<'a> {
    BuildingRoom(&'a BuildingRoom),
    ClearanceSpace(&'a ClearanceSpace),
    TrafficSpace(&'a TrafficSpace),
    TransportationSpaceKind(TransportationSpaceKindRef<'a>),
    AuxiliaryTrafficSpace(&'a AuxiliaryTrafficSpace),
}

impl<'a> From<&'a UnoccupiedSpaceKind> for UnoccupiedSpaceKindRef<'a> {
    fn from(item: &'a UnoccupiedSpaceKind) -> Self {
        match item {
            UnoccupiedSpaceKind::BuildingRoom(x) => Self::BuildingRoom(x),
            UnoccupiedSpaceKind::ClearanceSpace(x) => Self::ClearanceSpace(x),
            UnoccupiedSpaceKind::TrafficSpace(x) => Self::TrafficSpace(x),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => {
                Self::TransportationSpaceKind(x.into())
            }
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => Self::AuxiliaryTrafficSpace(x),
        }
    }
}

impl<'a> AsAbstractUnoccupiedSpace for UnoccupiedSpaceKindRef<'a> {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace {
        match self {
            Self::BuildingRoom(x) => x.abstract_unoccupied_space(),
            Self::ClearanceSpace(x) => x.abstract_unoccupied_space(),
            Self::TrafficSpace(x) => x.abstract_unoccupied_space(),
            Self::TransportationSpaceKind(x) => x.abstract_unoccupied_space(),
            Self::AuxiliaryTrafficSpace(x) => x.abstract_unoccupied_space(),
        }
    }
}
crate::impl_abstract_unoccupied_space_traits!(UnoccupiedSpaceKindRef<'_>);

impl<'a> HasFeatureType for UnoccupiedSpaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::BuildingRoom(_) => FeatureType::BuildingRoom,
            Self::ClearanceSpace(_) => FeatureType::ClearanceSpace,
            Self::TrafficSpace(_) => FeatureType::TrafficSpace,
            Self::TransportationSpaceKind(x) => x.feature_type(),
            Self::AuxiliaryTrafficSpace(_) => FeatureType::AuxiliaryTrafficSpace,
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_unoccupied_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::UnoccupiedSpaceKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::UnoccupiedSpaceKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_physical_space_kind_ref!(UnoccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_unoccupied_space_kind_ref!($variant, $variant);
    };
}
impl_from_for_unoccupied_space_kind_ref!(BuildingRoom);
impl_from_for_unoccupied_space_kind_ref!(ClearanceSpace);
impl_from_for_unoccupied_space_kind_ref!(TrafficSpace);
impl_from_for_unoccupied_space_kind_ref!(TransportationSpaceKind);
impl_from_for_unoccupied_space_kind_ref!(AuxiliaryTrafficSpace);

#[macro_export]
macro_rules! impl_try_from_for_unoccupied_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::UnoccupiedSpaceKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::UnoccupiedSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::UnoccupiedSpaceKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_physical_space_kind_ref!(UnoccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_unoccupied_space_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_unoccupied_space_kind_ref!(BuildingRoom);
impl_try_from_for_unoccupied_space_kind_ref!(ClearanceSpace);
impl_try_from_for_unoccupied_space_kind_ref!(TrafficSpace);
impl_try_from_for_unoccupied_space_kind_ref!(AuxiliaryTrafficSpace);

#[macro_export]
macro_rules! impl_try_from_unoccupied_space_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::UnoccupiedSpaceKindRef<'a>> for $EnumRef<'a> {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::UnoccupiedSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::UnoccupiedSpaceKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_physical_space_kind_ref_for_enum!(UnoccupiedSpaceKind, $EnumRef);
    };
}
impl_try_from_physical_space_kind_ref_for_enum!(UnoccupiedSpaceKind, UnoccupiedSpaceKindRef);
