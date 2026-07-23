use crate::impl_abstract_relief_component_mut_traits;
use crate::impl_abstract_relief_component_traits;
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::AsAbstractFeature;
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::relief::{
    AbstractReliefComponent, AsAbstractReliefComponent, AsAbstractReliefComponentMut, TinRelief,
};
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractReliefComponentKind {
    TinRelief(TinRelief),
}

impl AbstractReliefComponentKind {
    pub fn bounded_by(&self) -> Option<&Envelope> {
        match self {
            AbstractReliefComponentKind::TinRelief(x) => {
                egml::model::feature::AsAbstractFeature::bounded_by(x).and_then(|x| x.envelope())
            }
        }
    }
}

impl AsAbstractReliefComponent for AbstractReliefComponentKind {
    fn abstract_relief_component(&self) -> &AbstractReliefComponent {
        match self {
            AbstractReliefComponentKind::TinRelief(x) => x.abstract_relief_component(),
        }
    }
}

impl AsAbstractReliefComponentMut for AbstractReliefComponentKind {
    fn abstract_relief_component_mut(&mut self) -> &mut AbstractReliefComponent {
        match self {
            AbstractReliefComponentKind::TinRelief(x) => x.abstract_relief_component_mut(),
        }
    }
}

impl_abstract_relief_component_traits!(AbstractReliefComponentKind);
impl_abstract_relief_component_mut_traits!(AbstractReliefComponentKind);

impl HasFeatureType for AbstractReliefComponentKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::TinRelief(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_relief_component_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::relief::AbstractReliefComponentKind {
            fn from(x: $type) -> Self {
                $crate::model::relief::AbstractReliefComponentKind::$type(x)
            }
        }
        $crate::impl_from_for_space_boundary_kind!(AbstractReliefComponentKind, $type);
    };
}
impl_from_relief_component_kind!(TinRelief);

#[macro_export]
macro_rules! impl_try_from_relief_component_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::relief::AbstractReliefComponentKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::relief::AbstractReliefComponentKind) -> Result<Self, ()> {
                match x {
                    $crate::model::relief::AbstractReliefComponentKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_space_boundary_kind!(AbstractReliefComponentKind, $type);
    };
}
impl_try_from_relief_component_kind!(TinRelief);

impl IterFeatures for AbstractReliefComponentKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractReliefComponentKind::TinRelief(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractReliefComponentKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractReliefComponentKind::TinRelief(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractReliefComponentKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractReliefComponentKind::TinRelief(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractReliefComponentKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractReliefComponentKind::TinRelief(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractReliefComponentKind::TinRelief(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractReliefComponentKind::TinRelief(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractReliefComponentKind::TinRelief(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractReliefComponentKind::TinRelief(x) => x.apply_scale(scale),
        }
    }
}
