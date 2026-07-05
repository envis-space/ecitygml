use crate::impl_abstract_unoccupied_space_mut_traits;
use crate::impl_abstract_unoccupied_space_traits;
use crate::model::building::BuildingRoom;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::core::{
    AbstractUnoccupiedSpace, AsAbstractUnoccupiedSpace, AsAbstractUnoccupiedSpaceMut,
};
use crate::model::transportation::{
    AuxiliaryTrafficSpace, ClearanceSpace, TrafficSpace, TransportationSpaceKind,
};
use auto_enums::auto_enum;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum UnoccupiedSpaceKind {
    BuildingRoom(BuildingRoom),
    ClearanceSpace(ClearanceSpace),
    TrafficSpace(TrafficSpace),
    TransportationSpaceKind(TransportationSpaceKind),
    AuxiliaryTrafficSpace(AuxiliaryTrafficSpace),
}

impl UnoccupiedSpaceKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            UnoccupiedSpaceKind::BuildingRoom(x) => x.iter_features(),
            UnoccupiedSpaceKind::ClearanceSpace(x) => x.iter_features(),
            UnoccupiedSpaceKind::TrafficSpace(x) => x.iter_features(),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => x.iter_features(),
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            UnoccupiedSpaceKind::BuildingRoom(x) => x.for_each_feature_mut(f),
            UnoccupiedSpaceKind::ClearanceSpace(x) => x.for_each_feature_mut(f),
            UnoccupiedSpaceKind::TrafficSpace(x) => x.for_each_feature_mut(f),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => x.for_each_feature_mut(f),
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            UnoccupiedSpaceKind::BuildingRoom(x) => x.compute_envelope(),
            UnoccupiedSpaceKind::ClearanceSpace(x) => x.compute_envelope(),
            UnoccupiedSpaceKind::TrafficSpace(x) => x.compute_envelope(),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => x.compute_envelope(),
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            UnoccupiedSpaceKind::BuildingRoom(x) => x.recompute_bounding_shape(),
            UnoccupiedSpaceKind::ClearanceSpace(x) => x.recompute_bounding_shape(),
            UnoccupiedSpaceKind::TrafficSpace(x) => x.recompute_bounding_shape(),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => x.recompute_bounding_shape(),
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            UnoccupiedSpaceKind::BuildingRoom(x) => x.apply_transform(m),
            UnoccupiedSpaceKind::ClearanceSpace(x) => x.apply_transform(m),
            UnoccupiedSpaceKind::TrafficSpace(x) => x.apply_transform(m),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => x.apply_transform(m),
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractUnoccupiedSpace for UnoccupiedSpaceKind {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace {
        match self {
            UnoccupiedSpaceKind::BuildingRoom(x) => x.abstract_unoccupied_space(),
            UnoccupiedSpaceKind::ClearanceSpace(x) => x.abstract_unoccupied_space(),
            UnoccupiedSpaceKind::TrafficSpace(x) => x.abstract_unoccupied_space(),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => x.abstract_unoccupied_space(),
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.abstract_unoccupied_space(),
        }
    }
}

impl AsAbstractUnoccupiedSpaceMut for UnoccupiedSpaceKind {
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace {
        match self {
            UnoccupiedSpaceKind::BuildingRoom(x) => x.abstract_unoccupied_space_mut(),
            UnoccupiedSpaceKind::ClearanceSpace(x) => x.abstract_unoccupied_space_mut(),
            UnoccupiedSpaceKind::TrafficSpace(x) => x.abstract_unoccupied_space_mut(),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => x.abstract_unoccupied_space_mut(),
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.abstract_unoccupied_space_mut(),
        }
    }
}

impl_abstract_unoccupied_space_traits!(UnoccupiedSpaceKind);
impl_abstract_unoccupied_space_mut_traits!(UnoccupiedSpaceKind);

impl HasFeatureType for UnoccupiedSpaceKind {
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
macro_rules! impl_from_for_unoccupied_space_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::UnoccupiedSpaceKind {
            fn from(x: $type) -> Self {
                $crate::model::core::UnoccupiedSpaceKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_physical_space_kind!(UnoccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_unoccupied_space_kind!($variant, $variant);
    };
}
impl_from_for_unoccupied_space_kind!(BuildingRoom);
impl_from_for_unoccupied_space_kind!(ClearanceSpace);
impl_from_for_unoccupied_space_kind!(TrafficSpace);
impl_from_for_unoccupied_space_kind!(TransportationSpaceKind);
impl_from_for_unoccupied_space_kind!(AuxiliaryTrafficSpace);

#[macro_export]
macro_rules! impl_try_from_for_unoccupied_space_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::UnoccupiedSpaceKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::UnoccupiedSpaceKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::UnoccupiedSpaceKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_physical_space_kind!(UnoccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_unoccupied_space_kind!($variant, $variant);
    };
}
impl_try_from_for_unoccupied_space_kind!(BuildingRoom);
impl_try_from_for_unoccupied_space_kind!(ClearanceSpace);
impl_try_from_for_unoccupied_space_kind!(TrafficSpace);
impl_try_from_for_unoccupied_space_kind!(AuxiliaryTrafficSpace);
