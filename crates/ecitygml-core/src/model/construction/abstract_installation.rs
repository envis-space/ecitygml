use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractInstallation {
    pub(crate) abstract_occupied_space: AbstractOccupiedSpace,
}

impl AbstractInstallation {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_occupied_space(AbstractOccupiedSpace::new(id))
    }

    pub fn from_abstract_occupied_space(abstract_occupied_space: AbstractOccupiedSpace) -> Self {
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
                &<$type as $crate::model::construction::AsAbstractInstallation>::abstract_installation(self).abstract_occupied_space
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_installation_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_occupied_space_mut_traits!($type);

        impl $crate::model::core::AsAbstractOccupiedSpaceMut for $type {
            fn abstract_occupied_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractOccupiedSpace {
                &mut <$type as $crate::model::construction::AsAbstractInstallationMut>::abstract_installation_mut(self).abstract_occupied_space
            }
        }
    };
}

impl_abstract_installation_traits!(AbstractInstallation);
impl_abstract_installation_mut_traits!(AbstractInstallation);

impl IterFeatures for AbstractInstallation {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        self.abstract_occupied_space.iter_features()
    }
}

impl ForEachFeatureMut for AbstractInstallation {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_occupied_space.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for AbstractInstallation {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_occupied_space.compute_envelope()
    }
}

impl ApplyTransform for AbstractInstallation {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_occupied_space.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_occupied_space.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_occupied_space.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_occupied_space.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_occupied_space.apply_scale(scale);
    }
}
