use crate::model::building::{BuildingUnit, Storey};
use crate::model::core::{AbstractLogicalSpace, AsAbstractLogicalSpace, AsAbstractLogicalSpaceMut};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractBuildingSubdivision {
    pub abstract_logical_space: AbstractLogicalSpace,
}

impl AbstractBuildingSubdivision {
    pub fn new(abstract_logical_space: AbstractLogicalSpace) -> Self {
        Self {
            abstract_logical_space,
        }
    }
}

pub trait AsAbstractBuildingSubdivision: AsAbstractLogicalSpace {
    fn abstract_building_subdivision(&self) -> &AbstractBuildingSubdivision;
}

pub trait AsAbstractBuildingSubdivisionMut:
    AsAbstractLogicalSpaceMut + AsAbstractBuildingSubdivision
{
    fn abstract_building_subdivision_mut(&mut self) -> &mut AbstractBuildingSubdivision;
}

impl AsAbstractBuildingSubdivision for AbstractBuildingSubdivision {
    fn abstract_building_subdivision(&self) -> &AbstractBuildingSubdivision {
        self
    }
}

impl AsAbstractBuildingSubdivisionMut for AbstractBuildingSubdivision {
    fn abstract_building_subdivision_mut(&mut self) -> &mut AbstractBuildingSubdivision {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_building_subdivision_traits {
    ($type:ty) => {
        $crate::impl_abstract_logical_space_traits!($type);

        impl $crate::model::core::AsAbstractLogicalSpace for $type {
            fn abstract_logical_space(&self) -> &$crate::model::core::AbstractLogicalSpace {
                use $crate::model::building::AsAbstractBuildingSubdivision;
                &self.abstract_building_subdivision().abstract_logical_space
            }
        }

        impl $crate::model::core::AsAbstractLogicalSpaceMut for $type {
            fn abstract_logical_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractLogicalSpace {
                use $crate::model::building::AsAbstractBuildingSubdivisionMut;
                &mut self
                    .abstract_building_subdivision_mut()
                    .abstract_logical_space
            }
        }
    };
}

impl_abstract_building_subdivision_traits!(AbstractBuildingSubdivision);

#[derive(Debug, Clone, PartialEq)]
pub enum BuildingSubdivisionKind {
    BuildingUnit(BuildingUnit),
    Storey(Storey),
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

impl From<BuildingUnit> for BuildingSubdivisionKind {
    fn from(item: BuildingUnit) -> Self {
        BuildingSubdivisionKind::BuildingUnit(item)
    }
}

impl From<Storey> for BuildingSubdivisionKind {
    fn from(item: Storey) -> Self {
        BuildingSubdivisionKind::Storey(item)
    }
}
