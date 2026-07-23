use crate::impl_try_from_construction_kind_ref_mut_for_enum;
use crate::model::building::{
    AbstractBuilding, AbstractBuildingKind, AsAbstractBuilding, AsAbstractBuildingMut, Building,
    BuildingPart,
};
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractBuildingKindRefMut<'a> {
    Building(&'a mut Building),
    BuildingPart(&'a mut BuildingPart),
}

impl<'a> From<&'a mut AbstractBuildingKind> for AbstractBuildingKindRefMut<'a> {
    fn from(item: &'a mut AbstractBuildingKind) -> Self {
        match item {
            AbstractBuildingKind::Building(x) => Self::Building(x),
            AbstractBuildingKind::BuildingPart(x) => Self::BuildingPart(x),
        }
    }
}

impl<'a> AsAbstractBuilding for AbstractBuildingKindRefMut<'a> {
    fn abstract_building(&self) -> &AbstractBuilding {
        match self {
            Self::Building(x) => x.abstract_building(),
            Self::BuildingPart(x) => x.abstract_building(),
        }
    }
}

impl<'a> AsAbstractBuildingMut for AbstractBuildingKindRefMut<'a> {
    fn abstract_building_mut(&mut self) -> &mut AbstractBuilding {
        match self {
            Self::Building(x) => x.abstract_building_mut(),
            Self::BuildingPart(x) => x.abstract_building_mut(),
        }
    }
}
crate::impl_abstract_building_traits!(AbstractBuildingKindRefMut<'_>);
crate::impl_abstract_building_mut_traits!(AbstractBuildingKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractBuildingKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::Building(x) => x.feature_type(),
            Self::BuildingPart(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_building_kind_ref_mut {
    ($type:ident) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::building::refs::AbstractBuildingKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::building::refs::AbstractBuildingKindRefMut::$type(x.into())
            }
        }
        $crate::impl_from_for_construction_kind_ref_mut!(AbstractBuildingKind, $type);
    };
}
impl_from_building_kind_ref_mut!(Building);
impl_from_building_kind_ref_mut!(BuildingPart);

#[macro_export]
macro_rules! impl_try_from_building_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::building::refs::AbstractBuildingKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::building::refs::AbstractBuildingKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::building::refs::AbstractBuildingKindRefMut::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_construction_kind_ref_mut!(AbstractBuildingKind, $type);
    };
}
impl_try_from_building_kind_ref_mut!(Building);
impl_try_from_building_kind_ref_mut!(BuildingPart);
impl_try_from_construction_kind_ref_mut_for_enum!(AbstractBuildingKind, AbstractBuildingKindRefMut);

impl<'a> RecomputeBoundingShape for AbstractBuildingKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::Building(x) => x.recompute_bounding_shape(),
            Self::BuildingPart(x) => x.recompute_bounding_shape(),
        }
    }
}
