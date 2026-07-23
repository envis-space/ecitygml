use crate::impl_try_from_physical_space_kind_ref_mut_for_enum;
use crate::model::building::BuildingRoom;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::{
    AbstractUnoccupiedSpace, AbstractUnoccupiedSpaceKind, AsAbstractUnoccupiedSpace,
    AsAbstractUnoccupiedSpaceMut,
};
use crate::model::transportation::refs::AbstractTransportationSpaceKindRefMut;
use crate::model::transportation::{AbstractTransportationSpaceKind, Hole};
use crate::model::transportation::{AuxiliaryTrafficSpace, ClearanceSpace, TrafficSpace};
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractUnoccupiedSpaceKindRefMut<'a> {
    AuxiliaryTrafficSpace(&'a mut AuxiliaryTrafficSpace),
    BuildingRoom(&'a mut BuildingRoom),
    ClearanceSpace(&'a mut ClearanceSpace),
    Hole(&'a mut Hole),
    TrafficSpace(&'a mut TrafficSpace),
    AbstractTransportationSpaceKind(AbstractTransportationSpaceKindRefMut<'a>),
}

impl<'a> From<&'a mut AbstractUnoccupiedSpaceKind> for AbstractUnoccupiedSpaceKindRefMut<'a> {
    fn from(item: &'a mut AbstractUnoccupiedSpaceKind) -> Self {
        match item {
            AbstractUnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => Self::AuxiliaryTrafficSpace(x),
            AbstractUnoccupiedSpaceKind::BuildingRoom(x) => Self::BuildingRoom(x),
            AbstractUnoccupiedSpaceKind::ClearanceSpace(x) => Self::ClearanceSpace(x),
            AbstractUnoccupiedSpaceKind::Hole(x) => Self::Hole(x),
            AbstractUnoccupiedSpaceKind::TrafficSpace(x) => Self::TrafficSpace(x),
            AbstractUnoccupiedSpaceKind::AbstractTransportationSpaceKind(x) => {
                Self::AbstractTransportationSpaceKind(x.into())
            }
        }
    }
}

impl<'a> AsAbstractUnoccupiedSpace for AbstractUnoccupiedSpaceKindRefMut<'a> {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace {
        match self {
            Self::AuxiliaryTrafficSpace(x) => x.abstract_unoccupied_space(),
            Self::BuildingRoom(x) => x.abstract_unoccupied_space(),
            Self::ClearanceSpace(x) => x.abstract_unoccupied_space(),
            Self::Hole(x) => x.abstract_unoccupied_space(),
            Self::TrafficSpace(x) => x.abstract_unoccupied_space(),
            Self::AbstractTransportationSpaceKind(x) => x.abstract_unoccupied_space(),
        }
    }
}

impl<'a> AsAbstractUnoccupiedSpaceMut for AbstractUnoccupiedSpaceKindRefMut<'a> {
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace {
        match self {
            Self::AuxiliaryTrafficSpace(x) => x.abstract_unoccupied_space_mut(),
            Self::BuildingRoom(x) => x.abstract_unoccupied_space_mut(),
            Self::ClearanceSpace(x) => x.abstract_unoccupied_space_mut(),
            Self::Hole(x) => x.abstract_unoccupied_space_mut(),
            Self::TrafficSpace(x) => x.abstract_unoccupied_space_mut(),
            Self::AbstractTransportationSpaceKind(x) => x.abstract_unoccupied_space_mut(),
        }
    }
}
crate::impl_abstract_unoccupied_space_traits!(AbstractUnoccupiedSpaceKindRefMut<'_>);
crate::impl_abstract_unoccupied_space_mut_traits!(AbstractUnoccupiedSpaceKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractUnoccupiedSpaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AuxiliaryTrafficSpace(x) => x.feature_type(),
            Self::BuildingRoom(x) => x.feature_type(),
            Self::ClearanceSpace(x) => x.feature_type(),
            Self::Hole(x) => x.feature_type(),
            Self::TrafficSpace(x) => x.feature_type(),
            Self::AbstractTransportationSpaceKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_unoccupied_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::core::refs::AbstractUnoccupiedSpaceKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::AbstractUnoccupiedSpaceKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_physical_space_kind_ref_mut!(AbstractUnoccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_unoccupied_space_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_unoccupied_space_kind_ref_mut!(AuxiliaryTrafficSpace);
impl_from_for_unoccupied_space_kind_ref_mut!(BuildingRoom);
impl_from_for_unoccupied_space_kind_ref_mut!(ClearanceSpace);
impl_from_for_unoccupied_space_kind_ref_mut!(Hole);
impl_from_for_unoccupied_space_kind_ref_mut!(TrafficSpace);
impl_from_for_unoccupied_space_kind_ref_mut!(AbstractTransportationSpaceKind);

#[macro_export]
macro_rules! impl_try_from_for_unoccupied_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractUnoccupiedSpaceKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractUnoccupiedSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractUnoccupiedSpaceKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_physical_space_kind_ref_mut!(AbstractUnoccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_unoccupied_space_kind_ref_mut!($variant, $variant);
    };
}
impl_try_from_for_unoccupied_space_kind_ref_mut!(AuxiliaryTrafficSpace);
impl_try_from_for_unoccupied_space_kind_ref_mut!(BuildingRoom);
impl_try_from_for_unoccupied_space_kind_ref_mut!(ClearanceSpace);
impl_try_from_for_unoccupied_space_kind_ref_mut!(Hole);
impl_try_from_for_unoccupied_space_kind_ref_mut!(TrafficSpace);

#[macro_export]
macro_rules! impl_try_from_unoccupied_space_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractUnoccupiedSpaceKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractUnoccupiedSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractUnoccupiedSpaceKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_physical_space_kind_ref_mut_for_enum!(
            AbstractUnoccupiedSpaceKind,
            $EnumRefMut
        );
    };
}
impl_try_from_physical_space_kind_ref_mut_for_enum!(
    AbstractUnoccupiedSpaceKind,
    AbstractUnoccupiedSpaceKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractUnoccupiedSpaceKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::AuxiliaryTrafficSpace(x) => x.recompute_bounding_shape(),
            Self::BuildingRoom(x) => x.recompute_bounding_shape(),
            Self::ClearanceSpace(x) => x.recompute_bounding_shape(),
            Self::Hole(x) => x.recompute_bounding_shape(),
            Self::TrafficSpace(x) => x.recompute_bounding_shape(),
            Self::AbstractTransportationSpaceKind(x) => x.recompute_bounding_shape(),
        }
    }
}
