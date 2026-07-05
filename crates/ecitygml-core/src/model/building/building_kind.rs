use crate::impl_abstract_building_mut_traits;
use crate::impl_abstract_building_traits;
use crate::model::building::{
    AbstractBuilding, AsAbstractBuilding, AsAbstractBuildingMut, Building, BuildingPart,
};
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use auto_enums::auto_enum;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum BuildingKind {
    Building(Building),
    BuildingPart(BuildingPart),
}

impl BuildingKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            BuildingKind::Building(x) => x.iter_features(),
            BuildingKind::BuildingPart(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            BuildingKind::Building(x) => x.for_each_feature_mut(f),
            BuildingKind::BuildingPart(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            BuildingKind::Building(x) => x.compute_envelope(),
            BuildingKind::BuildingPart(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            BuildingKind::Building(x) => x.recompute_bounding_shape(),
            BuildingKind::BuildingPart(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            BuildingKind::Building(x) => x.apply_transform(m),
            BuildingKind::BuildingPart(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractBuilding for BuildingKind {
    fn abstract_building(&self) -> &AbstractBuilding {
        match self {
            BuildingKind::Building(x) => x.abstract_building(),
            BuildingKind::BuildingPart(x) => x.abstract_building(),
        }
    }
}

impl AsAbstractBuildingMut for BuildingKind {
    fn abstract_building_mut(&mut self) -> &mut AbstractBuilding {
        match self {
            BuildingKind::Building(x) => x.abstract_building_mut(),
            BuildingKind::BuildingPart(x) => x.abstract_building_mut(),
        }
    }
}

impl_abstract_building_traits!(BuildingKind);
impl_abstract_building_mut_traits!(BuildingKind);

impl HasFeatureType for BuildingKind {
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
        impl From<$type> for $crate::model::building::BuildingKind {
            fn from(x: $type) -> Self {
                $crate::model::building::BuildingKind::$type(x)
            }
        }
        $crate::impl_from_for_construction_kind!(BuildingKind, $type);
    };
}
impl_from_building_kind!(Building);
impl_from_building_kind!(BuildingPart);

#[macro_export]
macro_rules! impl_try_from_building_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::building::BuildingKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::building::BuildingKind) -> Result<Self, ()> {
                match x {
                    $crate::model::building::BuildingKind::$type(k) => k.try_into().map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_construction_kind!(BuildingKind, $type);
    };
}
impl_try_from_building_kind!(Building);
impl_try_from_building_kind!(BuildingPart);
