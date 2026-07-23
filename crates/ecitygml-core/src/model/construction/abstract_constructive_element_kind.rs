use crate::impl_abstract_constructive_element_mut_traits;
use crate::impl_abstract_constructive_element_traits;
use crate::model::building::BuildingConstructiveElement;
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::construction::{
    AbstractConstructiveElement, AsAbstractConstructiveElement, AsAbstractConstructiveElementMut,
};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractConstructiveElementKind {
    BuildingConstructiveElement(BuildingConstructiveElement),
}

impl AsAbstractConstructiveElement for AbstractConstructiveElementKind {
    fn abstract_constructive_element(&self) -> &AbstractConstructiveElement {
        match self {
            AbstractConstructiveElementKind::BuildingConstructiveElement(x) => {
                x.abstract_constructive_element()
            }
        }
    }
}

impl AsAbstractConstructiveElementMut for AbstractConstructiveElementKind {
    fn abstract_constructive_element_mut(&mut self) -> &mut AbstractConstructiveElement {
        match self {
            AbstractConstructiveElementKind::BuildingConstructiveElement(x) => {
                x.abstract_constructive_element_mut()
            }
        }
    }
}

impl_abstract_constructive_element_traits!(AbstractConstructiveElementKind);
impl_abstract_constructive_element_mut_traits!(AbstractConstructiveElementKind);

impl HasFeatureType for AbstractConstructiveElementKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::BuildingConstructiveElement(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_constructive_element_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::construction::AbstractConstructiveElementKind {
            fn from(x: $type) -> Self {
                $crate::model::construction::AbstractConstructiveElementKind::$type(x)
            }
        }
        $crate::impl_from_for_occupied_space_kind!(AbstractConstructiveElementKind, $type);
    };
}
impl_from_constructive_element_kind!(BuildingConstructiveElement);

#[macro_export]
macro_rules! impl_try_from_constructive_element_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::construction::AbstractConstructiveElementKind> for $type {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::AbstractConstructiveElementKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::AbstractConstructiveElementKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind!(AbstractConstructiveElementKind, $type);
    };
}
impl_try_from_constructive_element_kind!(BuildingConstructiveElement);

impl IterFeatures for AbstractConstructiveElementKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractConstructiveElementKind::BuildingConstructiveElement(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractConstructiveElementKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractConstructiveElementKind::BuildingConstructiveElement(x) => {
                x.for_each_feature_mut(f)
            }
        }
    }
}

impl ComputeEnvelope for AbstractConstructiveElementKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractConstructiveElementKind::BuildingConstructiveElement(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractConstructiveElementKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractConstructiveElementKind::BuildingConstructiveElement(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractConstructiveElementKind::BuildingConstructiveElement(x) => {
                x.apply_isometry(isometry)
            }
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractConstructiveElementKind::BuildingConstructiveElement(x) => {
                x.apply_translation(vector)
            }
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractConstructiveElementKind::BuildingConstructiveElement(x) => {
                x.apply_rotation(rotation)
            }
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractConstructiveElementKind::BuildingConstructiveElement(x) => x.apply_scale(scale),
        }
    }
}
