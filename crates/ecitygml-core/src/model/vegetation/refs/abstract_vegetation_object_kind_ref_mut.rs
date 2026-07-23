use crate::impl_try_from_occupied_space_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::vegetation::{
    AbstractVegetationObject, AbstractVegetationObjectKind, AsAbstractVegetationObject,
    AsAbstractVegetationObjectMut, PlantCover, SolitaryVegetationObject,
};
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractVegetationObjectKindRefMut<'a> {
    PlantCover(&'a mut PlantCover),
    SolitaryVegetationObject(&'a mut SolitaryVegetationObject),
}

impl<'a> From<&'a mut AbstractVegetationObjectKind> for AbstractVegetationObjectKindRefMut<'a> {
    fn from(item: &'a mut AbstractVegetationObjectKind) -> Self {
        match item {
            AbstractVegetationObjectKind::PlantCover(x) => Self::PlantCover(x),
            AbstractVegetationObjectKind::SolitaryVegetationObject(x) => {
                Self::SolitaryVegetationObject(x)
            }
        }
    }
}

impl<'a> AsAbstractVegetationObject for AbstractVegetationObjectKindRefMut<'a> {
    fn abstract_vegetation_object(&self) -> &AbstractVegetationObject {
        match self {
            Self::PlantCover(x) => x.abstract_vegetation_object(),
            Self::SolitaryVegetationObject(x) => x.abstract_vegetation_object(),
        }
    }
}

impl<'a> AsAbstractVegetationObjectMut for AbstractVegetationObjectKindRefMut<'a> {
    fn abstract_vegetation_object_mut(&mut self) -> &mut AbstractVegetationObject {
        match self {
            Self::PlantCover(x) => x.abstract_vegetation_object_mut(),
            Self::SolitaryVegetationObject(x) => x.abstract_vegetation_object_mut(),
        }
    }
}
crate::impl_abstract_vegetation_object_traits!(AbstractVegetationObjectKindRefMut<'_>);
crate::impl_abstract_vegetation_object_mut_traits!(AbstractVegetationObjectKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractVegetationObjectKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::PlantCover(x) => x.feature_type(),
            Self::SolitaryVegetationObject(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_vegetation_object_kind_ref_mut {
    ($type:ident) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::vegetation::refs::AbstractVegetationObjectKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::vegetation::refs::AbstractVegetationObjectKindRefMut::$type(x.into())
            }
        }
        $crate::impl_from_for_occupied_space_kind_ref_mut!(AbstractVegetationObjectKind, $type);
    };
}
impl_from_vegetation_object_kind_ref_mut!(PlantCover);
impl_from_vegetation_object_kind_ref_mut!(SolitaryVegetationObject);

#[macro_export]
macro_rules! impl_try_from_vegetation_object_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::vegetation::refs::AbstractVegetationObjectKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::vegetation::refs::AbstractVegetationObjectKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::vegetation::refs::AbstractVegetationObjectKindRefMut::$type(
                        k,
                    ) => k.try_into().map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind_ref_mut!(AbstractVegetationObjectKind, $type);
    };
}
impl_try_from_vegetation_object_kind_ref_mut!(PlantCover);
impl_try_from_vegetation_object_kind_ref_mut!(SolitaryVegetationObject);
impl_try_from_occupied_space_kind_ref_mut_for_enum!(
    AbstractVegetationObjectKind,
    AbstractVegetationObjectKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractVegetationObjectKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::PlantCover(x) => x.recompute_bounding_shape(),
            Self::SolitaryVegetationObject(x) => x.recompute_bounding_shape(),
        }
    }
}
