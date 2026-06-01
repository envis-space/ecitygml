use crate::impl_abstract_unoccupied_space_traits;
use crate::model::building::BuildingRoom;
use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};
use crate::model::core::{
    AbstractUnoccupiedSpace, AsAbstractUnoccupiedSpace, AsAbstractUnoccupiedSpaceMut,
};
use crate::model::transportation::{AuxiliaryTrafficSpace, TrafficSpace, TransportationSpaceKind};
use auto_enums::auto_enum;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum UnoccupiedSpaceKind {
    BuildingRoom(BuildingRoom),
    TrafficSpace(TrafficSpace),
    TransportationSpaceKind(TransportationSpaceKind),
    AuxiliaryTrafficSpace(AuxiliaryTrafficSpace),
}

impl UnoccupiedSpaceKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            UnoccupiedSpaceKind::BuildingRoom(x) => x.iter_features(),
            UnoccupiedSpaceKind::TrafficSpace(x) => x.iter_features(),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => x.iter_features(),
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            UnoccupiedSpaceKind::BuildingRoom(x) => x.for_each_feature_mut(f),
            UnoccupiedSpaceKind::TrafficSpace(x) => x.for_each_feature_mut(f),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => x.for_each_feature_mut(f),
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            UnoccupiedSpaceKind::BuildingRoom(x) => x.compute_envelope(),
            UnoccupiedSpaceKind::TrafficSpace(x) => x.compute_envelope(),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => x.compute_envelope(),
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            UnoccupiedSpaceKind::BuildingRoom(x) => x.recompute_bounding_shape(),
            UnoccupiedSpaceKind::TrafficSpace(x) => x.recompute_bounding_shape(),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => x.recompute_bounding_shape(),
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            UnoccupiedSpaceKind::BuildingRoom(x) => x.apply_transform(m),
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
            UnoccupiedSpaceKind::TrafficSpace(x) => x.abstract_unoccupied_space_mut(),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => x.abstract_unoccupied_space_mut(),
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.abstract_unoccupied_space_mut(),
        }
    }
}

impl_abstract_unoccupied_space_traits!(UnoccupiedSpaceKind);

impl<'a> From<&'a UnoccupiedSpaceKind> for FeatureRef<'a> {
    fn from(item: &'a UnoccupiedSpaceKind) -> Self {
        match item {
            UnoccupiedSpaceKind::BuildingRoom(x) => x.into(),
            UnoccupiedSpaceKind::TrafficSpace(x) => x.into(),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => x.into(),
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.into(),
        }
    }
}

impl<'a> TryFrom<&'a UnoccupiedSpaceKind> for TopLevelFeatureRef<'a> {
    type Error = ();
    fn try_from(item: &'a UnoccupiedSpaceKind) -> Result<Self, ()> {
        match item {
            UnoccupiedSpaceKind::BuildingRoom(_x) => Err(()),
            UnoccupiedSpaceKind::TrafficSpace(_x) => Err(()),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => x.try_into(),
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(_) => Err(()),
        }
    }
}

impl<'a> From<&'a mut UnoccupiedSpaceKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut UnoccupiedSpaceKind) -> Self {
        match item {
            UnoccupiedSpaceKind::BuildingRoom(x) => x.into(),
            UnoccupiedSpaceKind::TrafficSpace(x) => x.into(),
            UnoccupiedSpaceKind::TransportationSpaceKind(x) => x.into(),
            UnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.into(),
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
impl_from_for_unoccupied_space_kind!(TrafficSpace);
impl_from_for_unoccupied_space_kind!(TransportationSpaceKind);
impl_from_for_unoccupied_space_kind!(AuxiliaryTrafficSpace);
