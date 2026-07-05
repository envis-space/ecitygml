use crate::impl_try_from_physical_space_kind_ref_mut_for_enum;
use crate::model::building::BuildingRoom;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::{
    AbstractUnoccupiedSpace, AsAbstractUnoccupiedSpace, AsAbstractUnoccupiedSpaceMut,
    UnoccupiedSpaceKind,
};
use crate::model::transportation::TransportationSpaceKind;
use crate::model::transportation::refs::TransportationSpaceKindRefMut;
use crate::model::transportation::{AuxiliaryTrafficSpace, ClearanceSpace, TrafficSpace};

#[derive(Debug)]
pub enum UnoccupiedSpaceKindRefMut<'a> {
    BuildingRoom(&'a mut BuildingRoom),
    ClearanceSpace(&'a mut ClearanceSpace),
    TrafficSpace(&'a mut TrafficSpace),
    TransportationSpaceKind(TransportationSpaceKindRefMut<'a>),
    AuxiliaryTrafficSpace(&'a mut AuxiliaryTrafficSpace),
}

impl<'a> From<&'a mut UnoccupiedSpaceKind> for UnoccupiedSpaceKindRefMut<'a> {
    fn from(item: &'a mut UnoccupiedSpaceKind) -> Self {
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

impl<'a> AsAbstractUnoccupiedSpace for UnoccupiedSpaceKindRefMut<'a> {
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

impl<'a> AsAbstractUnoccupiedSpaceMut for UnoccupiedSpaceKindRefMut<'a> {
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace {
        match self {
            Self::BuildingRoom(x) => x.abstract_unoccupied_space_mut(),
            Self::ClearanceSpace(x) => x.abstract_unoccupied_space_mut(),
            Self::TrafficSpace(x) => x.abstract_unoccupied_space_mut(),
            Self::TransportationSpaceKind(x) => x.abstract_unoccupied_space_mut(),
            Self::AuxiliaryTrafficSpace(x) => x.abstract_unoccupied_space_mut(),
        }
    }
}
crate::impl_abstract_unoccupied_space_traits!(UnoccupiedSpaceKindRefMut<'_>);
crate::impl_abstract_unoccupied_space_mut_traits!(UnoccupiedSpaceKindRefMut<'_>);

impl<'a> UnoccupiedSpaceKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::BuildingRoom(x) => x.recompute_bounding_shape(),
            Self::ClearanceSpace(x) => x.recompute_bounding_shape(),
            Self::TrafficSpace(x) => x.recompute_bounding_shape(),
            Self::TransportationSpaceKind(x) => x.recompute_bounding_shape(),
            Self::AuxiliaryTrafficSpace(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for UnoccupiedSpaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::BuildingRoom(x) => x.feature_type(),
            Self::ClearanceSpace(x) => x.feature_type(),
            Self::TrafficSpace(x) => x.feature_type(),
            Self::TransportationSpaceKind(x) => x.feature_type(),
            Self::AuxiliaryTrafficSpace(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_unoccupied_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type> for $crate::model::core::refs::UnoccupiedSpaceKindRefMut<'a> {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::UnoccupiedSpaceKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_physical_space_kind_ref_mut!(UnoccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_unoccupied_space_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_unoccupied_space_kind_ref_mut!(BuildingRoom);
impl_from_for_unoccupied_space_kind_ref_mut!(ClearanceSpace);
impl_from_for_unoccupied_space_kind_ref_mut!(TrafficSpace);
impl_from_for_unoccupied_space_kind_ref_mut!(TransportationSpaceKind);
impl_from_for_unoccupied_space_kind_ref_mut!(AuxiliaryTrafficSpace);

#[macro_export]
macro_rules! impl_try_from_for_unoccupied_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::UnoccupiedSpaceKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::UnoccupiedSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::UnoccupiedSpaceKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_physical_space_kind_ref_mut!(UnoccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_unoccupied_space_kind_ref_mut!($variant, $variant);
    };
}
impl_try_from_for_unoccupied_space_kind_ref_mut!(BuildingRoom);
impl_try_from_for_unoccupied_space_kind_ref_mut!(ClearanceSpace);
impl_try_from_for_unoccupied_space_kind_ref_mut!(TrafficSpace);
impl_try_from_for_unoccupied_space_kind_ref_mut!(AuxiliaryTrafficSpace);

#[macro_export]
macro_rules! impl_try_from_unoccupied_space_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::UnoccupiedSpaceKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::UnoccupiedSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::UnoccupiedSpaceKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_physical_space_kind_ref_mut_for_enum!(
            UnoccupiedSpaceKind,
            $EnumRefMut
        );
    };
}
impl_try_from_physical_space_kind_ref_mut_for_enum!(UnoccupiedSpaceKind, UnoccupiedSpaceKindRefMut);
