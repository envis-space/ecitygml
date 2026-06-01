use crate::model::common::{FeatureRef, FeatureRefMut, LevelOfDetail};
use crate::model::core::{
    AbstractSpaceBoundary, AsAbstractSpaceBoundary, AsAbstractSpaceBoundaryMut,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractReliefComponent {
    pub(crate) abstract_space_boundary: AbstractSpaceBoundary,
    lod: LevelOfDetail,
}

impl AbstractReliefComponent {
    pub fn new(abstract_space_boundary: AbstractSpaceBoundary, lod: LevelOfDetail) -> Self {
        Self {
            abstract_space_boundary,
            lod,
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        self.abstract_space_boundary.iter_features()
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_space_boundary.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_space_boundary.compute_envelope()
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_space_boundary.apply_transform(m);
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
                use $crate::model::relief::AsAbstractReliefComponent;
                &self.abstract_relief_component().abstract_space_boundary
            }
        }

        impl $crate::model::core::AsAbstractSpaceBoundaryMut for $type {
            fn abstract_space_boundary_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractSpaceBoundary {
                use $crate::model::relief::AsAbstractReliefComponentMut;
                &mut self.abstract_relief_component_mut().abstract_space_boundary
            }
        }
    };
}

impl_abstract_relief_component_traits!(AbstractReliefComponent);
