use crate::impl_abstract_filling_element_traits;
use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::construction::{
    AbstractFillingElement, AsAbstractFillingElement, AsAbstractFillingElementMut, Door, Window,
};
use auto_enums::auto_enum;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum FillingElementKind {
    Door(Door),
    Window(Window),
}

impl FillingElementKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            FillingElementKind::Door(x) => x.iter_features(),
            FillingElementKind::Window(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            FillingElementKind::Door(x) => x.for_each_feature_mut(f),
            FillingElementKind::Window(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<egml::model::geometry::Envelope> {
        match self {
            FillingElementKind::Door(x) => x.compute_envelope(),
            FillingElementKind::Window(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            FillingElementKind::Door(x) => x.recompute_bounding_shape(),
            FillingElementKind::Window(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            FillingElementKind::Door(x) => x.apply_transform(m),
            FillingElementKind::Window(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractFillingElement for FillingElementKind {
    fn abstract_filling_element(&self) -> &AbstractFillingElement {
        match self {
            FillingElementKind::Door(element) => element.abstract_filling_element(),
            FillingElementKind::Window(element) => element.abstract_filling_element(),
        }
    }
}

impl AsAbstractFillingElementMut for FillingElementKind {
    fn abstract_filling_element_mut(&mut self) -> &mut AbstractFillingElement {
        match self {
            FillingElementKind::Door(element) => element.abstract_filling_element_mut(),
            FillingElementKind::Window(element) => element.abstract_filling_element_mut(),
        }
    }
}

impl_abstract_filling_element_traits!(FillingElementKind);

impl<'a> From<&'a FillingElementKind> for FeatureRef<'a> {
    fn from(item: &'a FillingElementKind) -> Self {
        match item {
            FillingElementKind::Door(x) => x.into(),
            FillingElementKind::Window(x) => x.into(),
        }
    }
}

impl<'a> From<&'a mut FillingElementKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut FillingElementKind) -> Self {
        match item {
            FillingElementKind::Door(x) => x.into(),
            FillingElementKind::Window(x) => x.into(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_filling_element_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::construction::FillingElementKind {
            fn from(x: $type) -> Self {
                $crate::model::construction::FillingElementKind::$type(x)
            }
        }
        $crate::impl_from_for_occupied_space_kind!(FillingElementKind, $type);
    };
}
impl_from_filling_element_kind!(Door);
impl_from_filling_element_kind!(Window);
