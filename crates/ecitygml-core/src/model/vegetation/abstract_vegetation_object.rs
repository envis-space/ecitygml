use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut,
};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractVegetationObject {
    pub(crate) abstract_occupied_space: AbstractOccupiedSpace,
}

impl AbstractVegetationObject {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_occupied_space(AbstractOccupiedSpace::new(id))
    }

    pub fn from_abstract_occupied_space(abstract_occupied_space: AbstractOccupiedSpace) -> Self {
        Self {
            abstract_occupied_space,
        }
    }
}
impl AbstractVegetationObject {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        self.abstract_occupied_space.iter_features()
    }
    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_occupied_space.for_each_feature_mut(f);
    }
    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_occupied_space.compute_envelope()
    }
    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_occupied_space.apply_transform(m);
    }
}

pub trait AsAbstractVegetationObject: AsAbstractOccupiedSpace {
    fn abstract_vegetation_object(&self) -> &AbstractVegetationObject;
}

pub trait AsAbstractVegetationObjectMut:
    AsAbstractOccupiedSpaceMut + AsAbstractVegetationObject
{
    fn abstract_vegetation_object_mut(&mut self) -> &mut AbstractVegetationObject;
}

impl AsAbstractVegetationObject for AbstractVegetationObject {
    fn abstract_vegetation_object(&self) -> &AbstractVegetationObject {
        self
    }
}

impl AsAbstractVegetationObjectMut for AbstractVegetationObject {
    fn abstract_vegetation_object_mut(&mut self) -> &mut AbstractVegetationObject {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_vegetation_object_traits {
    ($type:ty) => {
        $crate::impl_abstract_occupied_space_traits!($type);

        impl $crate::model::core::AsAbstractOccupiedSpace for $type {
            fn abstract_occupied_space(&self) -> &$crate::model::core::AbstractOccupiedSpace {
                use $crate::model::vegetation::AsAbstractVegetationObject;
                &self.abstract_vegetation_object().abstract_occupied_space
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_vegetation_object_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_occupied_space_mut_traits!($type);

        impl $crate::model::core::AsAbstractOccupiedSpaceMut for $type {
            fn abstract_occupied_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractOccupiedSpace {
                use $crate::model::vegetation::AsAbstractVegetationObjectMut;
                &mut self
                    .abstract_vegetation_object_mut()
                    .abstract_occupied_space
            }
        }
    };
}

impl_abstract_vegetation_object_traits!(AbstractVegetationObject);
impl_abstract_vegetation_object_mut_traits!(AbstractVegetationObject);
