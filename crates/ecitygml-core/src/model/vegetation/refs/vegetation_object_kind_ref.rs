use crate::impl_try_from_occupied_space_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::vegetation::{
    AbstractVegetationObject, AsAbstractVegetationObject, PlantCover, SolitaryVegetationObject,
    VegetationObjectKind,
};

#[derive(Debug, Clone, Copy)]
pub enum VegetationObjectKindRef<'a> {
    PlantCover(&'a PlantCover),
    SolitaryVegetationObject(&'a SolitaryVegetationObject),
}

impl<'a> From<&'a VegetationObjectKind> for VegetationObjectKindRef<'a> {
    fn from(item: &'a VegetationObjectKind) -> Self {
        match item {
            VegetationObjectKind::PlantCover(x) => Self::PlantCover(x),
            VegetationObjectKind::SolitaryVegetationObject(x) => Self::SolitaryVegetationObject(x),
        }
    }
}

impl<'a> AsAbstractVegetationObject for VegetationObjectKindRef<'a> {
    fn abstract_vegetation_object(&self) -> &AbstractVegetationObject {
        match self {
            Self::PlantCover(x) => x.abstract_vegetation_object(),
            Self::SolitaryVegetationObject(x) => x.abstract_vegetation_object(),
        }
    }
}
crate::impl_abstract_vegetation_object_traits!(VegetationObjectKindRef<'_>);

impl<'a> HasFeatureType for VegetationObjectKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::PlantCover(_) => FeatureType::PlantCover,
            Self::SolitaryVegetationObject(_) => FeatureType::SolitaryVegetationObject,
        }
    }
}

#[macro_export]
macro_rules! impl_from_vegetation_object_kind_ref {
    ($type:ident) => {
        impl<'a> From<&'a $type> for $crate::model::vegetation::refs::VegetationObjectKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::vegetation::refs::VegetationObjectKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_occupied_space_kind_ref!(VegetationObjectKind, $type);
    };
}
impl_from_vegetation_object_kind_ref!(PlantCover);
impl_from_vegetation_object_kind_ref!(SolitaryVegetationObject);

#[macro_export]
macro_rules! impl_try_from_vegetation_object_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::vegetation::refs::VegetationObjectKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::vegetation::refs::VegetationObjectKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::vegetation::refs::VegetationObjectKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind_ref!(VegetationObjectKind, $type);
    };
}
impl_try_from_vegetation_object_kind_ref!(PlantCover);
impl_try_from_vegetation_object_kind_ref!(SolitaryVegetationObject);
impl_try_from_occupied_space_kind_ref_for_enum!(VegetationObjectKind, VegetationObjectKindRef);
