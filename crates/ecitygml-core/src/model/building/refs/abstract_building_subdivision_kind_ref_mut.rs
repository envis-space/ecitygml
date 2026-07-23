use crate::impl_try_from_logical_space_kind_ref_mut_for_enum;
use crate::model::building::{
    AbstractBuildingSubdivision, AbstractBuildingSubdivisionKind, AsAbstractBuildingSubdivision,
    AsAbstractBuildingSubdivisionMut, BuildingUnit, Storey,
};
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractBuildingSubdivisionKindRefMut<'a> {
    BuildingUnit(&'a mut BuildingUnit),
    Storey(&'a mut Storey),
}

impl<'a> From<&'a mut AbstractBuildingSubdivisionKind>
    for AbstractBuildingSubdivisionKindRefMut<'a>
{
    fn from(item: &'a mut AbstractBuildingSubdivisionKind) -> Self {
        match item {
            AbstractBuildingSubdivisionKind::BuildingUnit(x) => Self::BuildingUnit(x),
            AbstractBuildingSubdivisionKind::Storey(x) => Self::Storey(x),
        }
    }
}

impl<'a> AsAbstractBuildingSubdivision for AbstractBuildingSubdivisionKindRefMut<'a> {
    fn abstract_building_subdivision(&self) -> &AbstractBuildingSubdivision {
        match self {
            Self::BuildingUnit(x) => x.abstract_building_subdivision(),
            Self::Storey(x) => x.abstract_building_subdivision(),
        }
    }
}

impl<'a> AsAbstractBuildingSubdivisionMut for AbstractBuildingSubdivisionKindRefMut<'a> {
    fn abstract_building_subdivision_mut(&mut self) -> &mut AbstractBuildingSubdivision {
        match self {
            Self::BuildingUnit(x) => x.abstract_building_subdivision_mut(),
            Self::Storey(x) => x.abstract_building_subdivision_mut(),
        }
    }
}
crate::impl_abstract_building_subdivision_traits!(AbstractBuildingSubdivisionKindRefMut<'_>);
crate::impl_abstract_building_subdivision_mut_traits!(AbstractBuildingSubdivisionKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractBuildingSubdivisionKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::BuildingUnit(x) => x.feature_type(),
            Self::Storey(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_building_subdivision_kind_ref_mut {
    ($type:ident) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::building::refs::AbstractBuildingSubdivisionKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::building::refs::AbstractBuildingSubdivisionKindRefMut::$type(
                    x.into(),
                )
            }
        }
        $crate::impl_from_for_logical_space_kind_ref_mut!(AbstractBuildingSubdivisionKind, $type);
    };
}
impl_from_building_subdivision_kind_ref_mut!(BuildingUnit);
impl_from_building_subdivision_kind_ref_mut!(Storey);

#[macro_export]
macro_rules! impl_try_from_building_subdivision_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::building::refs::AbstractBuildingSubdivisionKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::building::refs::AbstractBuildingSubdivisionKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::building::refs::AbstractBuildingSubdivisionKindRefMut::$type(
                        k,
                    ) => k.try_into().map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_logical_space_kind_ref_mut!(
            AbstractBuildingSubdivisionKind,
            $type
        );
    };
}
impl_try_from_building_subdivision_kind_ref_mut!(BuildingUnit);
impl_try_from_building_subdivision_kind_ref_mut!(Storey);
impl_try_from_logical_space_kind_ref_mut_for_enum!(
    AbstractBuildingSubdivisionKind,
    AbstractBuildingSubdivisionKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractBuildingSubdivisionKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::BuildingUnit(x) => x.recompute_bounding_shape(),
            Self::Storey(x) => x.recompute_bounding_shape(),
        }
    }
}
