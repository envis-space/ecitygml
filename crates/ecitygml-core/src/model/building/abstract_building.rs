use crate::model::construction::{
    AbstractConstruction, AsAbstractConstruction, AsAbstractConstructionMut,
};
use crate::model::core::AsAbstractOccupiedSpaceMut;
use egml::model::basic::Code;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractBuilding {
    pub abstract_construction: AbstractConstruction,
    pub(crate) function: Vec<Code>,
    pub(crate) usage: Vec<Code>,
    pub(crate) roof_type: Option<Code>,
    pub(crate) storeys_above_ground: Option<i64>,
    pub(crate) storeys_below_ground: Option<i64>,
}

impl AbstractBuilding {
    pub fn new(abstract_construction: AbstractConstruction) -> Self {
        Self {
            abstract_construction,
            function: Vec::new(),
            usage: Vec::new(),
            roof_type: None,
            storeys_above_ground: None,
            storeys_below_ground: None,
        }
    }
}
pub trait AsAbstractBuilding: AsAbstractConstruction {
    fn abstract_building(&self) -> &AbstractBuilding;

    fn function(&self) -> &Vec<Code> {
        &self.abstract_building().function
    }

    fn usage(&self) -> &Vec<Code> {
        &self.abstract_building().usage
    }

    fn roof_type(&self) -> &Option<Code> {
        &self.abstract_building().roof_type
    }

    fn storeys_above_ground(&self) -> Option<i64> {
        self.abstract_building().storeys_above_ground
    }

    fn storeys_below_ground(&self) -> Option<i64> {
        self.abstract_building().storeys_below_ground
    }
}

pub trait AsAbstractBuildingMut: AsAbstractConstructionMut + AsAbstractBuilding {
    fn abstract_building_mut(&mut self) -> &mut AbstractBuilding;

    fn set_function(&mut self, function: Vec<Code>) {
        self.abstract_building_mut().function = function;
    }

    fn set_usage(&mut self, usage: Vec<Code>) {
        self.abstract_building_mut().usage = usage;
    }

    fn set_roof_type(&mut self, roof_type: Option<Code>) {
        self.abstract_building_mut().roof_type = roof_type;
    }

    fn set_storeys_above_ground(&mut self, storeys_above_ground: Option<i64>) {
        self.abstract_building_mut().storeys_above_ground = storeys_above_ground;
    }

    fn set_storeys_below_ground(&mut self, storeys_below_ground: Option<i64>) {
        self.abstract_building_mut().storeys_below_ground = storeys_below_ground;
    }

    fn apply_transform(&mut self, m: &Isometry3<f64>) {
        AsAbstractOccupiedSpaceMut::apply_transform(self, m);
    }
}

impl AsAbstractBuilding for AbstractBuilding {
    fn abstract_building(&self) -> &AbstractBuilding {
        self
    }
}

impl AsAbstractBuildingMut for AbstractBuilding {
    fn abstract_building_mut(&mut self) -> &mut AbstractBuilding {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_building_traits {
    ($type:ty) => {
        $crate::impl_abstract_construction_traits!($type);

        impl $crate::model::construction::AsAbstractConstruction for $type {
            fn abstract_construction(&self) -> &$crate::model::construction::AbstractConstruction {
                use $crate::model::building::AsAbstractBuilding;
                &self.abstract_building().abstract_construction
            }
        }

        impl $crate::model::construction::AsAbstractConstructionMut for $type {
            fn abstract_construction_mut(
                &mut self,
            ) -> &mut $crate::model::construction::AbstractConstruction {
                use $crate::model::building::AsAbstractBuildingMut;
                &mut self.abstract_building_mut().abstract_construction
            }
        }
    };
}

impl_abstract_building_traits!(AbstractBuilding);
