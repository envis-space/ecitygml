use crate::impl_abstract_building_traits;
use crate::model::building::{
    AbstractBuilding, AsAbstractBuilding, AsAbstractBuildingMut, Building,
};
use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum BuildingKind {
    Building(Building),
    // BuildingPart(BuildingPart),
}

impl BuildingKind {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            BuildingKind::Building(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            BuildingKind::Building(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            BuildingKind::Building(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            BuildingKind::Building(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            BuildingKind::Building(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractBuilding for BuildingKind {
    fn abstract_building(&self) -> &AbstractBuilding {
        match self {
            BuildingKind::Building(x) => x.abstract_building(),
        }
    }
}

impl AsAbstractBuildingMut for BuildingKind {
    fn abstract_building_mut(&mut self) -> &mut AbstractBuilding {
        match self {
            BuildingKind::Building(x) => x.abstract_building_mut(),
        }
    }
}

impl_abstract_building_traits!(BuildingKind);

impl<'a> From<&'a BuildingKind> for FeatureRef<'a> {
    fn from(item: &'a BuildingKind) -> Self {
        match item {
            BuildingKind::Building(x) => x.into(),
        }
    }
}

impl<'a> TryFrom<&'a BuildingKind> for TopLevelFeatureRef<'a> {
    type Error = ();
    fn try_from(item: &'a BuildingKind) -> Result<Self, ()> {
        match item {
            BuildingKind::Building(x) => Ok(x.into()),
        }
    }
}

impl<'a> From<&'a mut BuildingKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut BuildingKind) -> Self {
        match item {
            BuildingKind::Building(x) => x.into(),
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
