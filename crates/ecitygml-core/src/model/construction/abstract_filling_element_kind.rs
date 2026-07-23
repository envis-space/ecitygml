use crate::impl_abstract_filling_element_mut_traits;
use crate::impl_abstract_filling_element_traits;
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::construction::{
    AbstractFillingElement, AsAbstractFillingElement, AsAbstractFillingElementMut, Door, Window,
};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractFillingElementKind {
    Door(Door),
    Window(Window),
}

impl AsAbstractFillingElement for AbstractFillingElementKind {
    fn abstract_filling_element(&self) -> &AbstractFillingElement {
        match self {
            AbstractFillingElementKind::Door(element) => element.abstract_filling_element(),
            AbstractFillingElementKind::Window(element) => element.abstract_filling_element(),
        }
    }
}

impl AsAbstractFillingElementMut for AbstractFillingElementKind {
    fn abstract_filling_element_mut(&mut self) -> &mut AbstractFillingElement {
        match self {
            AbstractFillingElementKind::Door(element) => element.abstract_filling_element_mut(),
            AbstractFillingElementKind::Window(element) => element.abstract_filling_element_mut(),
        }
    }
}

impl_abstract_filling_element_traits!(AbstractFillingElementKind);
impl_abstract_filling_element_mut_traits!(AbstractFillingElementKind);

impl HasFeatureType for AbstractFillingElementKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::Door(x) => x.feature_type(),
            Self::Window(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_filling_element_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::construction::AbstractFillingElementKind {
            fn from(x: $type) -> Self {
                $crate::model::construction::AbstractFillingElementKind::$type(x)
            }
        }
        $crate::impl_from_for_occupied_space_kind!(AbstractFillingElementKind, $type);
    };
}
impl_from_filling_element_kind!(Door);
impl_from_filling_element_kind!(Window);

#[macro_export]
macro_rules! impl_try_from_filling_element_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::construction::AbstractFillingElementKind> for $type {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::AbstractFillingElementKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::AbstractFillingElementKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind!(AbstractFillingElementKind, $type);
    };
}
impl_try_from_filling_element_kind!(Door);
impl_try_from_filling_element_kind!(Window);

impl IterFeatures for AbstractFillingElementKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractFillingElementKind::Door(x) => x.iter_features(),
            AbstractFillingElementKind::Window(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractFillingElementKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractFillingElementKind::Door(x) => x.for_each_feature_mut(f),
            AbstractFillingElementKind::Window(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractFillingElementKind {
    fn compute_envelope(&self) -> Option<egml::model::geometry::Envelope> {
        match self {
            AbstractFillingElementKind::Door(x) => x.compute_envelope(),
            AbstractFillingElementKind::Window(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractFillingElementKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractFillingElementKind::Door(x) => x.apply_transform(m),
            AbstractFillingElementKind::Window(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractFillingElementKind::Door(x) => x.apply_isometry(isometry),
            AbstractFillingElementKind::Window(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractFillingElementKind::Door(x) => x.apply_translation(vector),
            AbstractFillingElementKind::Window(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractFillingElementKind::Door(x) => x.apply_rotation(rotation),
            AbstractFillingElementKind::Window(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractFillingElementKind::Door(x) => x.apply_scale(scale),
            AbstractFillingElementKind::Window(x) => x.apply_scale(scale),
        }
    }
}
