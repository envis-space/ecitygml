use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractPhysicalSpace, AsAbstractPhysicalSpace, AsAbstractPhysicalSpaceMut,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractUnoccupiedSpace {
    pub(crate) abstract_physical_space: AbstractPhysicalSpace,
}

impl AbstractUnoccupiedSpace {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_physical_space(AbstractPhysicalSpace::new(id))
    }

    pub fn from_abstract_physical_space(abstract_physical_space: AbstractPhysicalSpace) -> Self {
        Self {
            abstract_physical_space,
        }
    }
}

pub trait AsAbstractUnoccupiedSpace: AsAbstractPhysicalSpace {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace;
}

pub trait AsAbstractUnoccupiedSpaceMut:
    AsAbstractPhysicalSpaceMut + AsAbstractUnoccupiedSpace
{
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
                &<$type as $crate::model::core::AsAbstractUnoccupiedSpace>::abstract_unoccupied_space(self).abstract_physical_space
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_unoccupied_space_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_physical_space_mut_traits!($type);

        impl $crate::model::core::AsAbstractPhysicalSpaceMut for $type {
            fn abstract_physical_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractPhysicalSpace {
                &mut <$type as $crate::model::core::AsAbstractUnoccupiedSpaceMut>::abstract_unoccupied_space_mut(self).abstract_physical_space
            }
        }
    };
}

impl_abstract_unoccupied_space_traits!(AbstractUnoccupiedSpace);
impl_abstract_unoccupied_space_mut_traits!(AbstractUnoccupiedSpace);

impl IterFeatures for AbstractUnoccupiedSpace {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        self.abstract_physical_space.iter_features()
    }
}

impl ForEachFeatureMut for AbstractUnoccupiedSpace {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_physical_space.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for AbstractUnoccupiedSpace {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_physical_space.compute_envelope()
    }
}

impl ApplyTransform for AbstractUnoccupiedSpace {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_physical_space.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_physical_space.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_physical_space.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_physical_space.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_physical_space.apply_scale(scale);
    }
}
