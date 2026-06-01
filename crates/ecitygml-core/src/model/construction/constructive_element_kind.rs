use crate::impl_abstract_constructive_element_traits;
use crate::model::building::BuildingConstructiveElement;
use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::construction::{
    AbstractConstructiveElement, AsAbstractConstructiveElement, AsAbstractConstructiveElementMut,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstructiveElementKind {
    BuildingConstructiveElement(BuildingConstructiveElement),
}

impl ConstructiveElementKind {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            ConstructiveElementKind::BuildingConstructiveElement(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            ConstructiveElementKind::BuildingConstructiveElement(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            ConstructiveElementKind::BuildingConstructiveElement(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            ConstructiveElementKind::BuildingConstructiveElement(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            ConstructiveElementKind::BuildingConstructiveElement(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractConstructiveElement for ConstructiveElementKind {
    fn abstract_constructive_element(&self) -> &AbstractConstructiveElement {
        match self {
            ConstructiveElementKind::BuildingConstructiveElement(x) => {
                x.abstract_constructive_element()
            }
        }
    }
}

impl AsAbstractConstructiveElementMut for ConstructiveElementKind {
    fn abstract_constructive_element_mut(&mut self) -> &mut AbstractConstructiveElement {
        match self {
            ConstructiveElementKind::BuildingConstructiveElement(x) => {
                x.abstract_constructive_element_mut()
            }
        }
    }
}

impl_abstract_constructive_element_traits!(ConstructiveElementKind);

impl<'a> From<&'a ConstructiveElementKind> for FeatureRef<'a> {
    fn from(item: &'a ConstructiveElementKind) -> Self {
        match item {
            ConstructiveElementKind::BuildingConstructiveElement(x) => x.into(),
        }
    }
}

impl<'a> From<&'a mut ConstructiveElementKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut ConstructiveElementKind) -> Self {
        match item {
            ConstructiveElementKind::BuildingConstructiveElement(x) => x.into(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_constructive_element_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::construction::ConstructiveElementKind {
            fn from(x: $type) -> Self {
                $crate::model::construction::ConstructiveElementKind::$type(x)
            }
        }
        $crate::impl_from_for_occupied_space_kind!(ConstructiveElementKind, $type);
    };
}
impl_from_constructive_element_kind!(BuildingConstructiveElement);
