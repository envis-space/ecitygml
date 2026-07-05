use crate::impl_try_from_occupied_space_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::vegetation::{
    AbstractVegetationObject, AsAbstractVegetationObject, AsAbstractVegetationObjectMut,
    PlantCover, SolitaryVegetationObject, VegetationObjectKind,
};

#[derive(Debug)]
pub enum VegetationObjectKindRefMut<'a> {
    PlantCover(&'a mut PlantCover),
    SolitaryVegetationObject(&'a mut SolitaryVegetationObject),
}

impl<'a> From<&'a mut VegetationObjectKind> for VegetationObjectKindRefMut<'a> {
    fn from(item: &'a mut VegetationObjectKind) -> Self {
        match item {
            VegetationObjectKind::PlantCover(x) => Self::PlantCover(x),
            VegetationObjectKind::SolitaryVegetationObject(x) => Self::SolitaryVegetationObject(x),
        }
    }
}

impl<'a> AsAbstractVegetationObject for VegetationObjectKindRefMut<'a> {
    fn abstract_vegetation_object(&self) -> &AbstractVegetationObject {
        match self {
            Self::PlantCover(x) => x.abstract_vegetation_object(),
            Self::SolitaryVegetationObject(x) => x.abstract_vegetation_object(),
        }
    }
}

impl<'a> AsAbstractVegetationObjectMut for VegetationObjectKindRefMut<'a> {
    fn abstract_vegetation_object_mut(&mut self) -> &mut AbstractVegetationObject {
        match self {
            Self::PlantCover(x) => x.abstract_vegetation_object_mut(),
            Self::SolitaryVegetationObject(x) => x.abstract_vegetation_object_mut(),
        }
    }
}
crate::impl_abstract_vegetation_object_traits!(VegetationObjectKindRefMut<'_>);
crate::impl_abstract_vegetation_object_mut_traits!(VegetationObjectKindRefMut<'_>);

impl<'a> VegetationObjectKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::PlantCover(x) => x.recompute_bounding_shape(),
            Self::SolitaryVegetationObject(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for VegetationObjectKindRefMut<'a> {
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
            for $crate::model::vegetation::refs::VegetationObjectKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::vegetation::refs::VegetationObjectKindRefMut::$type(x.into())
            }
        }
        $crate::impl_from_for_occupied_space_kind_ref_mut!(VegetationObjectKind, $type);
    };
}
impl_from_vegetation_object_kind_ref_mut!(PlantCover);
impl_from_vegetation_object_kind_ref_mut!(SolitaryVegetationObject);

#[macro_export]
macro_rules! impl_try_from_vegetation_object_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::vegetation::refs::VegetationObjectKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::vegetation::refs::VegetationObjectKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::vegetation::refs::VegetationObjectKindRefMut::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind_ref_mut!(VegetationObjectKind, $type);
    };
}
impl_try_from_vegetation_object_kind_ref_mut!(PlantCover);
impl_try_from_vegetation_object_kind_ref_mut!(SolitaryVegetationObject);
impl_try_from_occupied_space_kind_ref_mut_for_enum!(
    VegetationObjectKind,
    VegetationObjectKindRefMut
);
