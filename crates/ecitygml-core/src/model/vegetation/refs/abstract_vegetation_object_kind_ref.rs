use crate::impl_try_from_occupied_space_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::vegetation::{
    AbstractVegetationObject, AbstractVegetationObjectKind, AsAbstractVegetationObject, PlantCover,
    SolitaryVegetationObject,
};

#[derive(Debug, Clone, Copy)]
pub enum AbstractVegetationObjectKindRef<'a> {
    PlantCover(&'a PlantCover),
    SolitaryVegetationObject(&'a SolitaryVegetationObject),
}

impl<'a> From<&'a AbstractVegetationObjectKind> for AbstractVegetationObjectKindRef<'a> {
    fn from(item: &'a AbstractVegetationObjectKind) -> Self {
        match item {
            AbstractVegetationObjectKind::PlantCover(x) => Self::PlantCover(x),
            AbstractVegetationObjectKind::SolitaryVegetationObject(x) => {
                Self::SolitaryVegetationObject(x)
            }
        }
    }
}

impl<'a> AsAbstractVegetationObject for AbstractVegetationObjectKindRef<'a> {
    fn abstract_vegetation_object(&self) -> &AbstractVegetationObject {
        match self {
            Self::PlantCover(x) => x.abstract_vegetation_object(),
            Self::SolitaryVegetationObject(x) => x.abstract_vegetation_object(),
        }
    }
}
crate::impl_abstract_vegetation_object_traits!(AbstractVegetationObjectKindRef<'_>);

impl<'a> HasFeatureType for AbstractVegetationObjectKindRef<'a> {
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
        impl<'a> From<&'a $type>
            for $crate::model::vegetation::refs::AbstractVegetationObjectKindRef<'a>
        {
            fn from(x: &'a $type) -> Self {
                $crate::model::vegetation::refs::AbstractVegetationObjectKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_occupied_space_kind_ref!(AbstractVegetationObjectKind, $type);
    };
}
impl_from_vegetation_object_kind_ref!(PlantCover);
impl_from_vegetation_object_kind_ref!(SolitaryVegetationObject);

#[macro_export]
macro_rules! impl_try_from_vegetation_object_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::vegetation::refs::AbstractVegetationObjectKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::vegetation::refs::AbstractVegetationObjectKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::vegetation::refs::AbstractVegetationObjectKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind_ref!(AbstractVegetationObjectKind, $type);
    };
}
impl_try_from_vegetation_object_kind_ref!(PlantCover);
impl_try_from_vegetation_object_kind_ref!(SolitaryVegetationObject);
impl_try_from_occupied_space_kind_ref_for_enum!(
    AbstractVegetationObjectKind,
    AbstractVegetationObjectKindRef
);
