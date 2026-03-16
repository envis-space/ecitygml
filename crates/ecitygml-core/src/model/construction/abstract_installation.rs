use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractInstallation {
    pub abstract_occupied_space: AbstractOccupiedSpace,
}

impl AbstractInstallation {
    pub fn new(abstract_occupied_space: AbstractOccupiedSpace) -> Self {
        Self {
            abstract_occupied_space,
        }
    }
}

pub trait AsAbstractInstallation: AsAbstractOccupiedSpace {
    fn abstract_installation(&self) -> &AbstractInstallation;
}

pub trait AsAbstractInstallationMut: AsAbstractOccupiedSpaceMut + AsAbstractInstallation {
    fn abstract_installation_mut(&mut self) -> &mut AbstractInstallation;
}

impl AsAbstractInstallation for AbstractInstallation {
    fn abstract_installation(&self) -> &AbstractInstallation {
        self
    }
}

impl AsAbstractInstallationMut for AbstractInstallation {
    fn abstract_installation_mut(&mut self) -> &mut AbstractInstallation {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_installation_traits {
    ($type:ty) => {
        $crate::impl_abstract_occupied_space_traits!($type);

        impl $crate::model::core::AsAbstractOccupiedSpace for $type {
            fn abstract_occupied_space(&self) -> &$crate::model::core::AbstractOccupiedSpace {
                use $crate::model::construction::AsAbstractInstallation;
                &self.abstract_installation().abstract_occupied_space
            }
        }

        impl $crate::model::core::AsAbstractOccupiedSpaceMut for $type {
            fn abstract_occupied_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractOccupiedSpace {
                use $crate::model::construction::AsAbstractInstallationMut;
                &mut self.abstract_installation_mut().abstract_occupied_space
            }
        }
    };
}

impl_abstract_installation_traits!(AbstractInstallation);
