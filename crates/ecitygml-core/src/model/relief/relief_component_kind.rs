use crate::impl_abstract_relief_component_traits;
use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::core::AsAbstractFeature;
use crate::model::relief::{
    AbstractReliefComponent, AsAbstractReliefComponent, AsAbstractReliefComponentMut, TinRelief,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum ReliefComponentKind {
    TinRelief(TinRelief),
}

impl ReliefComponentKind {
    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            ReliefComponentKind::TinRelief(x) => x.compute_envelope(),
        }
    }
}

impl ReliefComponentKind {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            ReliefComponentKind::TinRelief(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            ReliefComponentKind::TinRelief(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            ReliefComponentKind::TinRelief(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            ReliefComponentKind::TinRelief(x) => x.apply_transform(m),
        }
    }

    pub fn bounded_by(&self) -> Option<&Envelope> {
        match self {
            ReliefComponentKind::TinRelief(x) => x.bounded_by(),
        }
    }
}

impl AsAbstractReliefComponent for ReliefComponentKind {
    fn abstract_relief_component(&self) -> &AbstractReliefComponent {
        match self {
            ReliefComponentKind::TinRelief(x) => x.abstract_relief_component(),
        }
    }
}

impl AsAbstractReliefComponentMut for ReliefComponentKind {
    fn abstract_relief_component_mut(&mut self) -> &mut AbstractReliefComponent {
        match self {
            ReliefComponentKind::TinRelief(x) => x.abstract_relief_component_mut(),
        }
    }
}

impl_abstract_relief_component_traits!(ReliefComponentKind);

impl<'a> From<&'a ReliefComponentKind> for FeatureRef<'a> {
    fn from(item: &'a ReliefComponentKind) -> Self {
        match item {
            ReliefComponentKind::TinRelief(x) => x.into(),
        }
    }
}

impl<'a> From<&'a mut ReliefComponentKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut ReliefComponentKind) -> Self {
        match item {
            ReliefComponentKind::TinRelief(x) => x.into(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_relief_component_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::relief::ReliefComponentKind {
            fn from(x: $type) -> Self {
                $crate::model::relief::ReliefComponentKind::$type(x)
            }
        }
        $crate::impl_from_for_space_boundary_kind!(ReliefComponentKind, $type);
    };
}
impl_from_relief_component_kind!(TinRelief);
