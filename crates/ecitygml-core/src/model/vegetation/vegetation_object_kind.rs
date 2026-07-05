use crate::impl_abstract_vegetation_object_mut_traits;
use crate::impl_abstract_vegetation_object_traits;
use auto_enums::auto_enum;

use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::vegetation::{
    AbstractVegetationObject, AsAbstractVegetationObject, AsAbstractVegetationObjectMut,
    PlantCover, SolitaryVegetationObject,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum VegetationObjectKind {
    PlantCover(PlantCover),
    SolitaryVegetationObject(SolitaryVegetationObject),
}

impl VegetationObjectKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            VegetationObjectKind::PlantCover(x) => x.iter_features(),
            VegetationObjectKind::SolitaryVegetationObject(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            VegetationObjectKind::PlantCover(x) => x.for_each_feature_mut(f),
            VegetationObjectKind::SolitaryVegetationObject(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            VegetationObjectKind::PlantCover(x) => x.compute_envelope(),
            VegetationObjectKind::SolitaryVegetationObject(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            VegetationObjectKind::PlantCover(x) => x.recompute_bounding_shape(),
            VegetationObjectKind::SolitaryVegetationObject(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            VegetationObjectKind::PlantCover(x) => x.apply_transform(m),
            VegetationObjectKind::SolitaryVegetationObject(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractVegetationObject for VegetationObjectKind {
    fn abstract_vegetation_object(&self) -> &AbstractVegetationObject {
        match self {
            VegetationObjectKind::PlantCover(x) => x.abstract_vegetation_object(),
            VegetationObjectKind::SolitaryVegetationObject(x) => x.abstract_vegetation_object(),
        }
    }
}

impl AsAbstractVegetationObjectMut for VegetationObjectKind {
    fn abstract_vegetation_object_mut(&mut self) -> &mut AbstractVegetationObject {
        match self {
            VegetationObjectKind::PlantCover(x) => x.abstract_vegetation_object_mut(),
            VegetationObjectKind::SolitaryVegetationObject(x) => x.abstract_vegetation_object_mut(),
        }
    }
}

impl_abstract_vegetation_object_traits!(VegetationObjectKind);
impl_abstract_vegetation_object_mut_traits!(VegetationObjectKind);

impl HasFeatureType for VegetationObjectKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::PlantCover(x) => x.feature_type(),
            Self::SolitaryVegetationObject(x) => x.feature_type(),
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
impl_from_vegetation_object_kind!(PlantCover);
impl_from_vegetation_object_kind!(SolitaryVegetationObject);

#[macro_export]
macro_rules! impl_try_from_vegetation_object_kind {
    ($type:ident) => {
        impl TryFrom<$crate::model::vegetation::VegetationObjectKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::vegetation::VegetationObjectKind) -> Result<Self, ()> {
                match x {
                    $crate::model::vegetation::VegetationObjectKind::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind!(VegetationObjectKind, $type);
    };
}
impl_try_from_vegetation_object_kind!(PlantCover);
impl_try_from_vegetation_object_kind!(SolitaryVegetationObject);
