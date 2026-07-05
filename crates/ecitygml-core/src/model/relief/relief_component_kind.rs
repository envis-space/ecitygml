use crate::impl_abstract_relief_component_mut_traits;
use crate::impl_abstract_relief_component_traits;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::AsAbstractFeature;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
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
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            ReliefComponentKind::TinRelief(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
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
impl_abstract_relief_component_mut_traits!(ReliefComponentKind);

impl HasFeatureType for ReliefComponentKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::TinRelief(x) => x.feature_type(),
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

#[macro_export]
macro_rules! impl_try_from_relief_component_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::relief::ReliefComponentKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::relief::ReliefComponentKind) -> Result<Self, ()> {
                match x {
                    $crate::model::relief::ReliefComponentKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_boundary_kind!(ReliefComponentKind, $type);
    };
}
impl_try_from_relief_component_kind!(TinRelief);
