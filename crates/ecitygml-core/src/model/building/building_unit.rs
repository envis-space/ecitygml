use crate::impl_abstract_building_subdivision_traits;
use crate::model::building::{
    AbstractBuildingSubdivision, AsAbstractBuildingSubdivision, AsAbstractBuildingSubdivisionMut,
};

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingUnit {
    pub abstract_building_subdivision: AbstractBuildingSubdivision,
}

impl BuildingUnit {
    pub fn new(abstract_building_subdivision: AbstractBuildingSubdivision) -> Self {
        Self {
            abstract_building_subdivision,
        }
    }
}

impl AsAbstractBuildingSubdivision for BuildingUnit {
    fn abstract_building_subdivision(&self) -> &AbstractBuildingSubdivision {
        &self.abstract_building_subdivision
    }
}

impl AsAbstractBuildingSubdivisionMut for BuildingUnit {
    fn abstract_building_subdivision_mut(&mut self) -> &mut AbstractBuildingSubdivision {
        &mut self.abstract_building_subdivision
    }
}

impl_abstract_building_subdivision_traits!(BuildingUnit);
