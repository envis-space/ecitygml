use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractConstructiveElement {
    pub abstract_occupied_space: AbstractOccupiedSpace,
    is_structural_element: Option<bool>,
}

impl AbstractConstructiveElement {
    pub fn new(abstract_occupied_space: AbstractOccupiedSpace) -> Self {
        Self {
            abstract_occupied_space,
            is_structural_element: None,
        }
    }
}

pub trait AsAbstractConstructiveElement: AsAbstractOccupiedSpace {
    fn abstract_constructive_element(&self) -> &AbstractConstructiveElement;
}

pub trait AsAbstractConstructiveElementMut:
    AsAbstractOccupiedSpaceMut + AsAbstractConstructiveElement
{
    fn abstract_constructive_element_mut(&mut self) -> &mut AbstractConstructiveElement;
}

impl AsAbstractConstructiveElement for AbstractConstructiveElement {
    fn abstract_constructive_element(&self) -> &AbstractConstructiveElement {
        self
    }
}

impl AsAbstractConstructiveElementMut for AbstractConstructiveElement {
    fn abstract_constructive_element_mut(&mut self) -> &mut AbstractConstructiveElement {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_constructive_element_traits {
    ($type:ty) => {
        $crate::impl_abstract_occupied_space_traits!($type);

        impl $crate::model::core::AsAbstractOccupiedSpace for $type {
            fn abstract_occupied_space(&self) -> &$crate::model::core::AbstractOccupiedSpace {
                use $crate::model::construction::AsAbstractConstructiveElement;
                &self.abstract_constructive_element().abstract_occupied_space
            }
        }

        impl $crate::model::core::AsAbstractOccupiedSpaceMut for $type {
            fn abstract_occupied_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractOccupiedSpace {
                use $crate::model::construction::AsAbstractConstructiveElementMut;
                &mut self
                    .abstract_constructive_element_mut()
                    .abstract_occupied_space
            }
        }
    };
}

impl_abstract_constructive_element_traits!(AbstractConstructiveElement);
