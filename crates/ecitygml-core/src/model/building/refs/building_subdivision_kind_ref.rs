use crate::impl_try_from_logical_space_kind_ref_for_enum;
use crate::model::building::{
    AbstractBuildingSubdivision, AsAbstractBuildingSubdivision, BuildingSubdivisionKind,
    BuildingUnit, Storey,
};
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;

#[derive(Debug, Clone, Copy)]
pub enum BuildingSubdivisionKindRef<'a> {
    BuildingUnit(&'a BuildingUnit),
    Storey(&'a Storey),
}

impl<'a> From<&'a BuildingSubdivisionKind> for BuildingSubdivisionKindRef<'a> {
    fn from(item: &'a BuildingSubdivisionKind) -> Self {
        match item {
            BuildingSubdivisionKind::BuildingUnit(x) => Self::BuildingUnit(x),
            BuildingSubdivisionKind::Storey(x) => Self::Storey(x),
        }
    }
}

impl<'a> AsAbstractBuildingSubdivision for BuildingSubdivisionKindRef<'a> {
    fn abstract_building_subdivision(&self) -> &AbstractBuildingSubdivision {
        match self {
            Self::BuildingUnit(x) => x.abstract_building_subdivision(),
            Self::Storey(x) => x.abstract_building_subdivision(),
        }
    }
}
crate::impl_abstract_building_subdivision_traits!(BuildingSubdivisionKindRef<'_>);

impl<'a> HasFeatureType for BuildingSubdivisionKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::BuildingUnit(_) => FeatureType::BuildingUnit,
            Self::Storey(_) => FeatureType::Storey,
        }
    }
}

#[macro_export]
macro_rules! impl_from_building_subdivision_kind_ref {
    ($type:ident) => {
        impl<'a> From<&'a $type> for $crate::model::building::refs::BuildingSubdivisionKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::building::refs::BuildingSubdivisionKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_logical_space_kind_ref!(BuildingSubdivisionKind, $type);
    };
}
impl_from_building_subdivision_kind_ref!(BuildingUnit);
impl_from_building_subdivision_kind_ref!(Storey);

#[macro_export]
macro_rules! impl_try_from_building_subdivision_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::building::refs::BuildingSubdivisionKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::building::refs::BuildingSubdivisionKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::building::refs::BuildingSubdivisionKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_logical_space_kind_ref!(BuildingSubdivisionKind, $type);
    };
}
impl_try_from_building_subdivision_kind_ref!(BuildingUnit);
impl_try_from_building_subdivision_kind_ref!(Storey);
impl_try_from_logical_space_kind_ref_for_enum!(BuildingSubdivisionKind, BuildingSubdivisionKindRef);
