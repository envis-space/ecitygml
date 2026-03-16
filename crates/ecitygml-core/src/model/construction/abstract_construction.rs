use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut, AsAbstractSpaceMut,
};
use chrono::NaiveDate;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractConstruction {
    pub abstract_occupied_space: AbstractOccupiedSpace,
    date_of_construction: Option<NaiveDate>,
    date_of_demolition: Option<NaiveDate>,
}

impl AbstractConstruction {
    pub fn new(abstract_occupied_space: AbstractOccupiedSpace) -> Self {
        Self {
            abstract_occupied_space,
            date_of_construction: None,
            date_of_demolition: None,
        }
    }
}

pub trait AsAbstractConstruction: AsAbstractOccupiedSpace {
    fn abstract_construction(&self) -> &AbstractConstruction;

    fn date_of_construction(&self) -> Option<&NaiveDate> {
        self.abstract_construction().date_of_construction.as_ref()
    }

    fn date_of_demolition(&self) -> Option<&NaiveDate> {
        self.abstract_construction().date_of_demolition.as_ref()
    }
}

pub trait AsAbstractConstructionMut: AsAbstractOccupiedSpaceMut + AsAbstractConstruction {
    fn abstract_construction_mut(&mut self) -> &mut AbstractConstruction;

    fn set_date_of_construction(&mut self, date_of_construction: Option<NaiveDate>) {
        self.abstract_construction_mut().date_of_construction = date_of_construction;
    }

    fn set_date_of_demolition(&mut self, date_of_demolition: Option<NaiveDate>) {
        self.abstract_construction_mut().date_of_demolition = date_of_demolition;
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
