use crate::impl_abstract_vegetation_object_traits;
use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};

use crate::model::vegetation::{
    AbstractVegetationObject, AsAbstractVegetationObject, AsAbstractVegetationObjectMut,
    SolitaryVegetationObject,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum VegetationObjectKind {
    SolitaryVegetationObject(SolitaryVegetationObject),
}

impl VegetationObjectKind {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            VegetationObjectKind::SolitaryVegetationObject(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            VegetationObjectKind::SolitaryVegetationObject(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            VegetationObjectKind::SolitaryVegetationObject(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            VegetationObjectKind::SolitaryVegetationObject(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            VegetationObjectKind::SolitaryVegetationObject(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractVegetationObject for VegetationObjectKind {
    fn abstract_vegetation_object(&self) -> &AbstractVegetationObject {
        match self {
            VegetationObjectKind::SolitaryVegetationObject(x) => x.abstract_vegetation_object(),
        }
    }
}

impl AsAbstractVegetationObjectMut for VegetationObjectKind {
    fn abstract_vegetation_object_mut(&mut self) -> &mut AbstractVegetationObject {
        match self {
            VegetationObjectKind::SolitaryVegetationObject(x) => x.abstract_vegetation_object_mut(),
        }
    }
}

impl_abstract_vegetation_object_traits!(VegetationObjectKind);

impl<'a> From<&'a VegetationObjectKind> for FeatureRef<'a> {
    fn from(item: &'a VegetationObjectKind) -> Self {
        match item {
            VegetationObjectKind::SolitaryVegetationObject(x) => x.into(),
        }
    }
}

impl<'a> From<&'a VegetationObjectKind> for TopLevelFeatureRef<'a> {
    fn from(item: &'a VegetationObjectKind) -> Self {
        match item {
            VegetationObjectKind::SolitaryVegetationObject(x) => x.into(),
        }
    }
}

impl<'a> From<&'a mut VegetationObjectKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut VegetationObjectKind) -> Self {
        match item {
            VegetationObjectKind::SolitaryVegetationObject(x) => x.into(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_vegetation_object_kind {
    ($type:ident) => {
        impl From<$type> for $crate::model::vegetation::VegetationObjectKind {
            fn from(x: $type) -> Self {
                $crate::model::vegetation::VegetationObjectKind::$type(x)
            }
        }
        $crate::impl_from_for_occupied_space_kind!(VegetationObjectKind, $type);
    };
}
impl_from_vegetation_object_kind!(SolitaryVegetationObject);
