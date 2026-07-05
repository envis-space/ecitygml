use crate::impl_abstract_building_subdivision_mut_traits;
use crate::impl_abstract_building_subdivision_traits;
use crate::model::building::{
    AbstractBuildingSubdivision, AsAbstractBuildingSubdivision, AsAbstractBuildingSubdivisionMut,
    BuildingUnit, Storey,
};
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum BuildingSubdivisionKind {
    BuildingUnit(BuildingUnit),
    Storey(Storey),
}

impl BuildingSubdivisionKind {
    #[auto_enums::auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            BuildingSubdivisionKind::BuildingUnit(x) => x.iter_features(),
            BuildingSubdivisionKind::Storey(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            BuildingSubdivisionKind::BuildingUnit(x) => x.for_each_feature_mut(f),
            BuildingSubdivisionKind::Storey(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<egml::model::geometry::Envelope> {
        match self {
            BuildingSubdivisionKind::BuildingUnit(x) => x.compute_envelope(),
            BuildingSubdivisionKind::Storey(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            BuildingSubdivisionKind::BuildingUnit(x) => x.recompute_bounding_shape(),
            BuildingSubdivisionKind::Storey(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            BuildingSubdivisionKind::BuildingUnit(x) => x.apply_transform(m),
            BuildingSubdivisionKind::Storey(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractBuildingSubdivision for BuildingSubdivisionKind {
    fn abstract_building_subdivision(&self) -> &AbstractBuildingSubdivision {
        match self {
            BuildingSubdivisionKind::BuildingUnit(subdivision) => {
                subdivision.abstract_building_subdivision()
            }
            BuildingSubdivisionKind::Storey(subdivision) => {
                subdivision.abstract_building_subdivision()
            }
        }
    }
}

impl AsAbstractBuildingSubdivisionMut for BuildingSubdivisionKind {
    fn abstract_building_subdivision_mut(&mut self) -> &mut AbstractBuildingSubdivision {
        match self {
            BuildingSubdivisionKind::BuildingUnit(subdivision) => {
                subdivision.abstract_building_subdivision_mut()
            }
            BuildingSubdivisionKind::Storey(subdivision) => {
                subdivision.abstract_building_subdivision_mut()
            }
        }
    }
}

impl_abstract_building_subdivision_traits!(BuildingSubdivisionKind);
impl_abstract_building_subdivision_mut_traits!(BuildingSubdivisionKind);

impl HasFeatureType for BuildingSubdivisionKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::BuildingUnit(x) => x.feature_type(),
            Self::Storey(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_building_subdivision_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::building::BuildingSubdivisionKind {
            fn from(x: $type) -> Self {
                $crate::model::building::BuildingSubdivisionKind::$type(x)
            }
        }
        $crate::impl_from_for_logical_space_kind!(BuildingSubdivisionKind, $type);
    };
}
impl_from_building_subdivision_kind!(BuildingUnit);
impl_from_building_subdivision_kind!(Storey);

#[macro_export]
macro_rules! impl_try_from_building_subdivision_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::building::BuildingSubdivisionKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::building::BuildingSubdivisionKind) -> Result<Self, ()> {
                match x {
                    $crate::model::building::BuildingSubdivisionKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_logical_space_kind!(BuildingSubdivisionKind, $type);
    };
}
impl_try_from_building_subdivision_kind!(BuildingUnit);
impl_try_from_building_subdivision_kind!(Storey);
