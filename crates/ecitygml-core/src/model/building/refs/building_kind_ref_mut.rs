use crate::impl_try_from_construction_kind_ref_mut_for_enum;
use crate::model::building::{
    AbstractBuilding, AsAbstractBuilding, AsAbstractBuildingMut, Building, BuildingKind,
    BuildingPart,
};
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;

#[derive(Debug)]
pub enum BuildingKindRefMut<'a> {
    Building(&'a mut Building),
    BuildingPart(&'a mut BuildingPart),
}

impl<'a> From<&'a mut BuildingKind> for BuildingKindRefMut<'a> {
    fn from(item: &'a mut BuildingKind) -> Self {
        match item {
            BuildingKind::Building(x) => Self::Building(x),
            BuildingKind::BuildingPart(x) => Self::BuildingPart(x),
        }
    }
}

impl<'a> AsAbstractBuilding for BuildingKindRefMut<'a> {
    fn abstract_building(&self) -> &AbstractBuilding {
        match self {
            Self::Building(x) => x.abstract_building(),
            Self::BuildingPart(x) => x.abstract_building(),
        }
    }
}

impl<'a> AsAbstractBuildingMut for BuildingKindRefMut<'a> {
    fn abstract_building_mut(&mut self) -> &mut AbstractBuilding {
        match self {
            Self::Building(x) => x.abstract_building_mut(),
            Self::BuildingPart(x) => x.abstract_building_mut(),
        }
    }
}
crate::impl_abstract_building_traits!(BuildingKindRefMut<'_>);
crate::impl_abstract_building_mut_traits!(BuildingKindRefMut<'_>);

impl<'a> BuildingKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::Building(x) => x.recompute_bounding_shape(),
            Self::BuildingPart(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for BuildingKindRefMut<'a> {
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
        impl<'a> From<&'a mut $type> for $crate::model::building::refs::BuildingKindRefMut<'a> {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::building::refs::BuildingKindRefMut::$type(x.into())
            }
        }
        $crate::impl_from_for_construction_kind_ref_mut!(BuildingKind, $type);
    };
}
impl_from_building_kind_ref_mut!(Building);
impl_from_building_kind_ref_mut!(BuildingPart);

#[macro_export]
macro_rules! impl_try_from_building_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::building::refs::BuildingKindRefMut<'a>> for &'a mut $type {
            type Error = ();
            fn try_from(
                x: $crate::model::building::refs::BuildingKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::building::refs::BuildingKindRefMut::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_construction_kind_ref_mut!(BuildingKind, $type);
    };
}
impl_try_from_building_kind_ref_mut!(Building);
impl_try_from_building_kind_ref_mut!(BuildingPart);
impl_try_from_construction_kind_ref_mut_for_enum!(BuildingKind, BuildingKindRefMut);
