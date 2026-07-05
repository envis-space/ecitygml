use crate::impl_try_from_construction_kind_ref_for_enum;
use crate::model::building::{
    AbstractBuilding, AsAbstractBuilding, Building, BuildingKind, BuildingPart,
};
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;

#[derive(Debug, Clone, Copy)]
pub enum BuildingKindRef<'a> {
    Building(&'a Building),
    BuildingPart(&'a BuildingPart),
}

impl<'a> From<&'a BuildingKind> for BuildingKindRef<'a> {
    fn from(item: &'a BuildingKind) -> Self {
        match item {
            BuildingKind::Building(x) => Self::Building(x),
            BuildingKind::BuildingPart(x) => Self::BuildingPart(x),
        }
    }
}

impl<'a> AsAbstractBuilding for BuildingKindRef<'a> {
    fn abstract_building(&self) -> &AbstractBuilding {
        match self {
            Self::Building(x) => x.abstract_building(),
            Self::BuildingPart(x) => x.abstract_building(),
        }
    }
}
crate::impl_abstract_building_traits!(BuildingKindRef<'_>);

impl<'a> HasFeatureType for BuildingKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::Building(_) => FeatureType::Building,
            Self::BuildingPart(_) => FeatureType::BuildingPart,
        }
    }
}

#[macro_export]
macro_rules! impl_from_building_kind_ref {
    ($type:ident) => {
        impl<'a> From<&'a $type> for $crate::model::building::refs::BuildingKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::building::refs::BuildingKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_construction_kind_ref!(BuildingKind, $type);
    };
}
impl_from_building_kind_ref!(Building);
impl_from_building_kind_ref!(BuildingPart);

#[macro_export]
macro_rules! impl_try_from_building_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::building::refs::BuildingKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(x: $crate::model::building::refs::BuildingKindRef<'a>) -> Result<Self, ()> {
                match x {
                    $crate::model::building::refs::BuildingKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_construction_kind_ref!(BuildingKind, $type);
    };
}
impl_try_from_building_kind_ref!(Building);
impl_try_from_building_kind_ref!(BuildingPart);
impl_try_from_construction_kind_ref_for_enum!(BuildingKind, BuildingKindRef);
