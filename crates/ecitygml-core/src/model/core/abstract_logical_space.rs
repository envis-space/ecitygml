use crate::model::core::{AbstractSpace, AsAbstractSpace, AsAbstractSpaceMut};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractLogicalSpace {
    pub(crate) abstract_space: AbstractSpace,
}

impl AbstractLogicalSpace {
    pub fn new(abstract_space: AbstractSpace) -> Self {
        AbstractLogicalSpace { abstract_space }
    }
}

pub trait AsAbstractLogicalSpace: AsAbstractSpace {
    fn abstract_logical_space(&self) -> &AbstractLogicalSpace;
}

pub trait AsAbstractLogicalSpaceMut: AsAbstractSpaceMut + AsAbstractLogicalSpace {
    fn abstract_logical_space_mut(&mut self) -> &mut AbstractLogicalSpace;
}

impl AsAbstractLogicalSpace for AbstractLogicalSpace {
    fn abstract_logical_space(&self) -> &AbstractLogicalSpace {
        self
    }
}

impl AsAbstractLogicalSpaceMut for AbstractLogicalSpace {
    fn abstract_logical_space_mut(&mut self) -> &mut AbstractLogicalSpace {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_logical_space_traits {
    ($type:ty) => {
        $crate::impl_abstract_space_traits!($type);

        impl $crate::model::core::AsAbstractSpace for $type {
            fn abstract_space(&self) -> &$crate::model::core::AbstractSpace {
                use $crate::model::core::AsAbstractLogicalSpace;
                &self.abstract_logical_space().abstract_space
            }
        }

        impl $crate::model::core::AsAbstractSpaceMut for $type {
            fn abstract_space_mut(&mut self) -> &mut $crate::model::core::AbstractSpace {
                use $crate::model::core::AsAbstractLogicalSpaceMut;
                &mut self.abstract_logical_space_mut().abstract_space
            }
        }
    };
}

impl_abstract_logical_space_traits!(AbstractLogicalSpace);
