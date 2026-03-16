use crate::model::core::{AbstractPhysicalSpace, AsAbstractSpace, AsAbstractSpaceMut};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractUnoccupiedSpace {
    pub(crate) abstract_physical_space: AbstractPhysicalSpace,
}

impl AbstractUnoccupiedSpace {
    pub fn new(abstract_physical_space: AbstractPhysicalSpace) -> Self {
        Self {
            abstract_physical_space,
        }
    }
}

pub trait AsAbstractUnoccupiedSpace: AsAbstractSpace {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace;
}

pub trait AsAbstractUnoccupiedSpaceMut: AsAbstractSpaceMut + AsAbstractUnoccupiedSpace {
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace;
}
impl AsAbstractUnoccupiedSpace for AbstractUnoccupiedSpace {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace {
        self
    }
}

impl AsAbstractUnoccupiedSpaceMut for AbstractUnoccupiedSpace {
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_unoccupied_space_traits {
    ($type:ty) => {
        $crate::impl_abstract_physical_space_traits!($type);

        impl $crate::model::core::AsAbstractPhysicalSpace for $type {
            fn abstract_physical_space(&self) -> &$crate::model::core::AbstractPhysicalSpace {
                use $crate::model::core::AsAbstractUnoccupiedSpace;
                &self.abstract_unoccupied_space().abstract_physical_space
            }
        }

        impl $crate::model::core::AsAbstractPhysicalSpaceMut for $type {
            fn abstract_physical_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractPhysicalSpace {
                use $crate::model::core::AsAbstractUnoccupiedSpaceMut;
                &mut self.abstract_unoccupied_space_mut().abstract_physical_space
            }
        }
    };
}

impl_abstract_unoccupied_space_traits!(AbstractUnoccupiedSpace);
