use crate::impl_abstract_unoccupied_space_mut_traits;
use crate::impl_abstract_unoccupied_space_traits;
use crate::model::building::BuildingRoom;
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractUnoccupiedSpace, AsAbstractUnoccupiedSpace, AsAbstractUnoccupiedSpaceMut,
};
use crate::model::transportation::{
    AbstractTransportationSpaceKind, AuxiliaryTrafficSpace, ClearanceSpace, Hole, TrafficSpace,
};
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractUnoccupiedSpaceKind {
    AuxiliaryTrafficSpace(AuxiliaryTrafficSpace),
    BuildingRoom(BuildingRoom),
    ClearanceSpace(ClearanceSpace),
    Hole(Hole),
    TrafficSpace(TrafficSpace),
    AbstractTransportationSpaceKind(AbstractTransportationSpaceKind),
}

impl AsAbstractUnoccupiedSpace for AbstractUnoccupiedSpaceKind {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace {
        match self {
            AbstractUnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.abstract_unoccupied_space(),
            AbstractUnoccupiedSpaceKind::BuildingRoom(x) => x.abstract_unoccupied_space(),
            AbstractUnoccupiedSpaceKind::ClearanceSpace(x) => x.abstract_unoccupied_space(),
            AbstractUnoccupiedSpaceKind::Hole(x) => x.abstract_unoccupied_space(),
            AbstractUnoccupiedSpaceKind::TrafficSpace(x) => x.abstract_unoccupied_space(),
            AbstractUnoccupiedSpaceKind::AbstractTransportationSpaceKind(x) => {
                x.abstract_unoccupied_space()
            }
        }
    }
}

impl AsAbstractUnoccupiedSpaceMut for AbstractUnoccupiedSpaceKind {
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace {
        match self {
            AbstractUnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => {
                x.abstract_unoccupied_space_mut()
            }
            AbstractUnoccupiedSpaceKind::BuildingRoom(x) => x.abstract_unoccupied_space_mut(),
            AbstractUnoccupiedSpaceKind::ClearanceSpace(x) => x.abstract_unoccupied_space_mut(),
            AbstractUnoccupiedSpaceKind::Hole(x) => x.abstract_unoccupied_space_mut(),
            AbstractUnoccupiedSpaceKind::TrafficSpace(x) => x.abstract_unoccupied_space_mut(),
            AbstractUnoccupiedSpaceKind::AbstractTransportationSpaceKind(x) => {
                x.abstract_unoccupied_space_mut()
            }
        }
    }
}

impl_abstract_unoccupied_space_traits!(AbstractUnoccupiedSpaceKind);
impl_abstract_unoccupied_space_mut_traits!(AbstractUnoccupiedSpaceKind);

impl HasFeatureType for AbstractUnoccupiedSpaceKind {
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
macro_rules! impl_from_for_unoccupied_space_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::AbstractUnoccupiedSpaceKind {
            fn from(x: $type) -> Self {
                $crate::model::core::AbstractUnoccupiedSpaceKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_physical_space_kind!(AbstractUnoccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_unoccupied_space_kind!($variant, $variant);
    };
}
impl_from_for_unoccupied_space_kind!(AuxiliaryTrafficSpace);
impl_from_for_unoccupied_space_kind!(BuildingRoom);
impl_from_for_unoccupied_space_kind!(ClearanceSpace);
impl_from_for_unoccupied_space_kind!(Hole);
impl_from_for_unoccupied_space_kind!(TrafficSpace);
impl_from_for_unoccupied_space_kind!(AbstractTransportationSpaceKind);

#[macro_export]
macro_rules! impl_try_from_for_unoccupied_space_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::AbstractUnoccupiedSpaceKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::AbstractUnoccupiedSpaceKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::AbstractUnoccupiedSpaceKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_physical_space_kind!(AbstractUnoccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_unoccupied_space_kind!($variant, $variant);
    };
}
impl_try_from_for_unoccupied_space_kind!(AuxiliaryTrafficSpace);
impl_try_from_for_unoccupied_space_kind!(BuildingRoom);
impl_try_from_for_unoccupied_space_kind!(ClearanceSpace);
impl_try_from_for_unoccupied_space_kind!(Hole);
impl_try_from_for_unoccupied_space_kind!(TrafficSpace);

impl IterFeatures for AbstractUnoccupiedSpaceKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractUnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.iter_features(),
            AbstractUnoccupiedSpaceKind::BuildingRoom(x) => x.iter_features(),
            AbstractUnoccupiedSpaceKind::ClearanceSpace(x) => x.iter_features(),
            AbstractUnoccupiedSpaceKind::Hole(x) => x.iter_features(),
            AbstractUnoccupiedSpaceKind::TrafficSpace(x) => x.iter_features(),
            AbstractUnoccupiedSpaceKind::AbstractTransportationSpaceKind(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractUnoccupiedSpaceKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractUnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.for_each_feature_mut(f),
            AbstractUnoccupiedSpaceKind::BuildingRoom(x) => x.for_each_feature_mut(f),
            AbstractUnoccupiedSpaceKind::ClearanceSpace(x) => x.for_each_feature_mut(f),
            AbstractUnoccupiedSpaceKind::Hole(x) => x.for_each_feature_mut(f),
            AbstractUnoccupiedSpaceKind::TrafficSpace(x) => x.for_each_feature_mut(f),
            AbstractUnoccupiedSpaceKind::AbstractTransportationSpaceKind(x) => {
                x.for_each_feature_mut(f)
            }
        }
    }
}

impl ComputeEnvelope for AbstractUnoccupiedSpaceKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractUnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.compute_envelope(),
            AbstractUnoccupiedSpaceKind::BuildingRoom(x) => x.compute_envelope(),
            AbstractUnoccupiedSpaceKind::ClearanceSpace(x) => x.compute_envelope(),
            AbstractUnoccupiedSpaceKind::Hole(x) => x.compute_envelope(),
            AbstractUnoccupiedSpaceKind::TrafficSpace(x) => x.compute_envelope(),
            AbstractUnoccupiedSpaceKind::AbstractTransportationSpaceKind(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractUnoccupiedSpaceKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractUnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.apply_transform(m),
            AbstractUnoccupiedSpaceKind::BuildingRoom(x) => x.apply_transform(m),
            AbstractUnoccupiedSpaceKind::ClearanceSpace(x) => x.apply_transform(m),
            AbstractUnoccupiedSpaceKind::Hole(x) => x.apply_transform(m),
            AbstractUnoccupiedSpaceKind::TrafficSpace(x) => x.apply_transform(m),
            AbstractUnoccupiedSpaceKind::AbstractTransportationSpaceKind(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractUnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.apply_isometry(isometry),
            AbstractUnoccupiedSpaceKind::BuildingRoom(x) => x.apply_isometry(isometry),
            AbstractUnoccupiedSpaceKind::ClearanceSpace(x) => x.apply_isometry(isometry),
            AbstractUnoccupiedSpaceKind::Hole(x) => x.apply_isometry(isometry),
            AbstractUnoccupiedSpaceKind::TrafficSpace(x) => x.apply_isometry(isometry),
            AbstractUnoccupiedSpaceKind::AbstractTransportationSpaceKind(x) => {
                x.apply_isometry(isometry)
            }
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractUnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.apply_translation(vector),
            AbstractUnoccupiedSpaceKind::BuildingRoom(x) => x.apply_translation(vector),
            AbstractUnoccupiedSpaceKind::ClearanceSpace(x) => x.apply_translation(vector),
            AbstractUnoccupiedSpaceKind::Hole(x) => x.apply_translation(vector),
            AbstractUnoccupiedSpaceKind::TrafficSpace(x) => x.apply_translation(vector),
            AbstractUnoccupiedSpaceKind::AbstractTransportationSpaceKind(x) => {
                x.apply_translation(vector)
            }
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractUnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.apply_rotation(rotation),
            AbstractUnoccupiedSpaceKind::BuildingRoom(x) => x.apply_rotation(rotation),
            AbstractUnoccupiedSpaceKind::ClearanceSpace(x) => x.apply_rotation(rotation),
            AbstractUnoccupiedSpaceKind::Hole(x) => x.apply_rotation(rotation),
            AbstractUnoccupiedSpaceKind::TrafficSpace(x) => x.apply_rotation(rotation),
            AbstractUnoccupiedSpaceKind::AbstractTransportationSpaceKind(x) => {
                x.apply_rotation(rotation)
            }
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractUnoccupiedSpaceKind::AuxiliaryTrafficSpace(x) => x.apply_scale(scale),
            AbstractUnoccupiedSpaceKind::BuildingRoom(x) => x.apply_scale(scale),
            AbstractUnoccupiedSpaceKind::ClearanceSpace(x) => x.apply_scale(scale),
            AbstractUnoccupiedSpaceKind::Hole(x) => x.apply_scale(scale),
            AbstractUnoccupiedSpaceKind::TrafficSpace(x) => x.apply_scale(scale),
            AbstractUnoccupiedSpaceKind::AbstractTransportationSpaceKind(x) => x.apply_scale(scale),
        }
    }
}
