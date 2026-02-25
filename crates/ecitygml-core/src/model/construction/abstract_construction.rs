use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut, AsAbstractSpaceMut,
};
use chrono::{DateTime, FixedOffset};
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractConstruction {
    pub abstract_occupied_space: AbstractOccupiedSpace,
    creation_date: Option<DateTime<FixedOffset>>,
    demolition_date: Option<DateTime<FixedOffset>>,
}

impl AbstractConstruction {
    pub fn new(abstract_occupied_space: AbstractOccupiedSpace) -> Self {
        Self {
            abstract_occupied_space,
            creation_date: None,
            demolition_date: None,
        }
    }
}

pub trait AsAbstractConstruction: AsAbstractOccupiedSpace {
    fn abstract_construction(&self) -> &AbstractConstruction;

    fn creation_date(&self) -> Option<&DateTime<FixedOffset>> {
        self.abstract_construction().creation_date.as_ref()
    }

    fn demolition_date(&self) -> Option<&DateTime<FixedOffset>> {
        self.abstract_construction().demolition_date.as_ref()
    }
}

pub trait AsAbstractConstructionMut: AsAbstractOccupiedSpaceMut + AsAbstractConstruction {
    fn abstract_construction_mut(&mut self) -> &mut AbstractConstruction;

    fn set_creation_date(&mut self, creation_date: Option<DateTime<FixedOffset>>) {
        self.abstract_construction_mut().creation_date = creation_date;
    }

    fn set_demolition_date(&mut self, demolition_date: Option<DateTime<FixedOffset>>) {
        self.abstract_construction_mut().demolition_date = demolition_date;
    }

    fn apply_transform(&mut self, m: &Isometry3<f64>) {
        AsAbstractOccupiedSpaceMut::apply_transform(self, m);
    }
}

impl AsAbstractConstruction for AbstractConstruction {
    fn abstract_construction(&self) -> &AbstractConstruction {
        self
    }
}

impl AsAbstractConstructionMut for AbstractConstruction {
    fn abstract_construction_mut(&mut self) -> &mut AbstractConstruction {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_construction_traits {
    ($type:ty) => {
        $crate::impl_abstract_occupied_space_traits!($type);

        impl $crate::model::core::AsAbstractOccupiedSpace for $type {
            fn abstract_occupied_space(&self) -> &$crate::model::core::AbstractOccupiedSpace {
                use $crate::model::construction::AsAbstractConstruction;
                &self.abstract_construction().abstract_occupied_space
            }
        }

        impl $crate::model::core::AsAbstractOccupiedSpaceMut for $type {
            fn abstract_occupied_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractOccupiedSpace {
                use $crate::model::construction::AsAbstractConstructionMut;
                &mut self.abstract_construction_mut().abstract_occupied_space
            }
        }
    };
}

impl_abstract_construction_traits!(AbstractConstruction);
