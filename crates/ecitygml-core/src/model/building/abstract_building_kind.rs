use crate::impl_abstract_building_mut_traits;
use crate::impl_abstract_building_traits;
use crate::model::building::{
    AbstractBuilding, AsAbstractBuilding, AsAbstractBuildingMut, Building, BuildingPart,
};
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractBuildingKind {
    Building(Building),
    BuildingPart(BuildingPart),
}

impl AsAbstractBuilding for AbstractBuildingKind {
    fn abstract_building(&self) -> &AbstractBuilding {
        match self {
            AbstractBuildingKind::Building(x) => x.abstract_building(),
            AbstractBuildingKind::BuildingPart(x) => x.abstract_building(),
        }
    }
}

impl AsAbstractBuildingMut for AbstractBuildingKind {
    fn abstract_building_mut(&mut self) -> &mut AbstractBuilding {
        match self {
            AbstractBuildingKind::Building(x) => x.abstract_building_mut(),
            AbstractBuildingKind::BuildingPart(x) => x.abstract_building_mut(),
        }
    }
}

impl_abstract_building_traits!(AbstractBuildingKind);
impl_abstract_building_mut_traits!(AbstractBuildingKind);

impl HasFeatureType for AbstractBuildingKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::Building(x) => x.feature_type(),
            Self::BuildingPart(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_building_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::building::AbstractBuildingKind {
            fn from(x: $type) -> Self {
                $crate::model::building::AbstractBuildingKind::$type(x)
            }
        }
        $crate::impl_from_for_construction_kind!(AbstractBuildingKind, $type);
    };
}
impl_from_building_kind!(Building);
impl_from_building_kind!(BuildingPart);

#[macro_export]
macro_rules! impl_try_from_building_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::building::AbstractBuildingKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::building::AbstractBuildingKind) -> Result<Self, ()> {
                match x {
                    $crate::model::building::AbstractBuildingKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_construction_kind!(AbstractBuildingKind, $type);
    };
}
impl_try_from_building_kind!(Building);
impl_try_from_building_kind!(BuildingPart);

impl IterFeatures for AbstractBuildingKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractBuildingKind::Building(x) => x.iter_features(),
            AbstractBuildingKind::BuildingPart(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractBuildingKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractBuildingKind::Building(x) => x.for_each_feature_mut(f),
            AbstractBuildingKind::BuildingPart(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractBuildingKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractBuildingKind::Building(x) => x.compute_envelope(),
            AbstractBuildingKind::BuildingPart(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractBuildingKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractBuildingKind::Building(x) => x.apply_transform(m),
            AbstractBuildingKind::BuildingPart(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractBuildingKind::Building(x) => x.apply_isometry(isometry),
            AbstractBuildingKind::BuildingPart(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractBuildingKind::Building(x) => x.apply_translation(vector),
            AbstractBuildingKind::BuildingPart(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractBuildingKind::Building(x) => x.apply_rotation(rotation),
            AbstractBuildingKind::BuildingPart(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractBuildingKind::Building(x) => x.apply_scale(scale),
            AbstractBuildingKind::BuildingPart(x) => x.apply_scale(scale),
        }
    }
}
