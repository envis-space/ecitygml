use crate::impl_abstract_filling_element_mut_traits;
use crate::impl_abstract_filling_element_traits;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::construction::{
    AbstractFillingElement, AsAbstractFillingElement, AsAbstractFillingElementMut, Door, Window,
};
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use auto_enums::auto_enum;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum FillingElementKind {
    Door(Door),
    Window(Window),
}

impl FillingElementKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            FillingElementKind::Door(x) => x.iter_features(),
            FillingElementKind::Window(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
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
impl_abstract_filling_element_mut_traits!(FillingElementKind);

impl HasFeatureType for FillingElementKind {
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

#[macro_export]
macro_rules! impl_try_from_filling_element_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::construction::FillingElementKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::construction::FillingElementKind) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::FillingElementKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind!(FillingElementKind, $type);
    };
}
impl_try_from_filling_element_kind!(Door);
impl_try_from_filling_element_kind!(Window);
