use crate::impl_try_from_construction_kind_ref_for_enum;
use crate::model::building::{
    AbstractBuilding, AbstractBuildingKind, AsAbstractBuilding, Building, BuildingPart,
};
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;

#[derive(Debug, Clone, Copy)]
pub enum AbstractBuildingKindRef<'a> {
    Building(&'a Building),
    BuildingPart(&'a BuildingPart),
}

impl<'a> From<&'a AbstractBuildingKind> for AbstractBuildingKindRef<'a> {
    fn from(item: &'a AbstractBuildingKind) -> Self {
        match item {
            AbstractBuildingKind::Building(x) => Self::Building(x),
            AbstractBuildingKind::BuildingPart(x) => Self::BuildingPart(x),
        }
    }
}

impl<'a> AsAbstractBuilding for AbstractBuildingKindRef<'a> {
    fn abstract_building(&self) -> &AbstractBuilding {
        match self {
            Self::Building(x) => x.abstract_building(),
            Self::BuildingPart(x) => x.abstract_building(),
        }
    }
}
crate::impl_abstract_building_traits!(AbstractBuildingKindRef<'_>);

impl<'a> HasFeatureType for AbstractBuildingKindRef<'a> {
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
        impl<'a> From<&'a $type> for $crate::model::building::refs::AbstractBuildingKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::building::refs::AbstractBuildingKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_construction_kind_ref!(AbstractBuildingKind, $type);
    };
}
impl_from_building_kind_ref!(Building);
impl_from_building_kind_ref!(BuildingPart);

#[macro_export]
macro_rules! impl_try_from_building_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::building::refs::AbstractBuildingKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::building::refs::AbstractBuildingKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::building::refs::AbstractBuildingKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_construction_kind_ref!(AbstractBuildingKind, $type);
    };
}
impl_try_from_building_kind_ref!(Building);
impl_try_from_building_kind_ref!(BuildingPart);
impl_try_from_construction_kind_ref_for_enum!(AbstractBuildingKind, AbstractBuildingKindRef);
