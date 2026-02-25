use crate::model::common::LevelOfDetail;
use crate::model::core::{
    AbstractSpaceBoundary, AsAbstractFeature, AsAbstractSpaceBoundary, AsAbstractSpaceBoundaryMut,
};
use crate::model::relief::TinRelief;
use egml::model::geometry::Envelope;

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

#[derive(Debug, Clone, PartialEq)]
pub enum ReliefComponentKind {
    TinRelief(TinRelief),
}

impl From<TinRelief> for ReliefComponentKind {
    fn from(item: TinRelief) -> Self {
        ReliefComponentKind::TinRelief(item)
    }
}

impl ReliefComponentKind {
    pub fn refresh_bounded_by_recursive(&mut self) {
        match self {
            ReliefComponentKind::TinRelief(x) => x.refresh_bounded_by_recursive(),
        }
    }

    pub fn bounded_by(&self) -> Option<&Envelope> {
        match self {
            ReliefComponentKind::TinRelief(x) => x.bounded_by(),
        }
    }
}
