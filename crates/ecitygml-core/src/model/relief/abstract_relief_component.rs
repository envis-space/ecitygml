use crate::model::common::{ForEachFeatureMut, IterFeatures, LevelOfDetail};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractSpaceBoundary, AsAbstractSpaceBoundary, AsAbstractSpaceBoundaryMut,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractReliefComponent {
    pub(crate) abstract_space_boundary: AbstractSpaceBoundary,
    lod: LevelOfDetail,
}

impl AbstractReliefComponent {
    pub fn new(id: Id, lod: LevelOfDetail) -> Self {
        Self::from_abstract_space_boundary(AbstractSpaceBoundary::new(id), lod)
    }

    pub fn from_abstract_space_boundary(
        abstract_space_boundary: AbstractSpaceBoundary,
        lod: LevelOfDetail,
    ) -> Self {
        Self {
            abstract_space_boundary,
            lod,
        }
    }
}

pub trait AsAbstractReliefComponent: AsAbstractSpaceBoundary {
    fn abstract_relief_component(&self) -> &AbstractReliefComponent;

    fn lod(&self) -> LevelOfDetail {
        self.abstract_relief_component().lod
    }
}

pub trait AsAbstractReliefComponentMut:
    AsAbstractSpaceBoundaryMut + AsAbstractReliefComponent
{
    fn abstract_relief_component_mut(&mut self) -> &mut AbstractReliefComponent;
}

impl AsAbstractReliefComponent for AbstractReliefComponent {
    fn abstract_relief_component(&self) -> &AbstractReliefComponent {
        self
    }
}

impl AsAbstractReliefComponentMut for AbstractReliefComponent {
    fn abstract_relief_component_mut(&mut self) -> &mut AbstractReliefComponent {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_relief_component_traits {
    ($type:ty) => {
        $crate::impl_abstract_space_boundary_traits!($type);

        impl $crate::model::core::AsAbstractSpaceBoundary for $type {
            fn abstract_space_boundary(&self) -> &$crate::model::core::AbstractSpaceBoundary {
                &<$type as $crate::model::relief::AsAbstractReliefComponent>::abstract_relief_component(self).abstract_space_boundary
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_relief_component_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_space_boundary_mut_traits!($type);

        impl $crate::model::core::AsAbstractSpaceBoundaryMut for $type {
            fn abstract_space_boundary_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractSpaceBoundary {
                &mut <$type as $crate::model::relief::AsAbstractReliefComponentMut>::abstract_relief_component_mut(self).abstract_space_boundary
            }
        }
    };
}

impl_abstract_relief_component_traits!(AbstractReliefComponent);
impl_abstract_relief_component_mut_traits!(AbstractReliefComponent);

impl IterFeatures for AbstractReliefComponent {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        self.abstract_space_boundary.iter_features()
    }
}

impl ForEachFeatureMut for AbstractReliefComponent {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_space_boundary.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for AbstractReliefComponent {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_space_boundary.compute_envelope()
    }
}

impl ApplyTransform for AbstractReliefComponent {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_space_boundary.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_space_boundary.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_space_boundary.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_space_boundary.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_space_boundary.apply_scale(scale);
    }
}
