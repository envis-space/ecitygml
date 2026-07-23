use crate::impl_try_from_feature_with_lifespan_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::{AbstractSpaceBoundaryKindRefMut, AbstractSpaceKindRefMut};
use crate::model::core::{
    AbstractCityObject, AbstractCityObjectKind, AsAbstractCityObject, AsAbstractCityObjectMut,
};
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractCityObjectKindRefMut<'a> {
    AbstractSpaceKind(AbstractSpaceKindRefMut<'a>),
    AbstractSpaceBoundaryKind(AbstractSpaceBoundaryKindRefMut<'a>),
}

impl<'a> From<&'a mut AbstractCityObjectKind> for AbstractCityObjectKindRefMut<'a> {
    fn from(item: &'a mut AbstractCityObjectKind) -> Self {
        match item {
            AbstractCityObjectKind::AbstractSpaceKind(x) => Self::AbstractSpaceKind(x.into()),
            AbstractCityObjectKind::AbstractSpaceBoundaryKind(x) => {
                Self::AbstractSpaceBoundaryKind(x.into())
            }
        }
    }
}

impl<'a> AsAbstractCityObject for AbstractCityObjectKindRefMut<'a> {
    fn abstract_city_object(&self) -> &AbstractCityObject {
        match self {
            Self::AbstractSpaceKind(x) => x.abstract_city_object(),
            Self::AbstractSpaceBoundaryKind(x) => x.abstract_city_object(),
        }
    }
}

impl<'a> AsAbstractCityObjectMut for AbstractCityObjectKindRefMut<'a> {
    fn abstract_city_object_mut(&mut self) -> &mut AbstractCityObject {
        match self {
            Self::AbstractSpaceKind(x) => x.abstract_city_object_mut(),
            Self::AbstractSpaceBoundaryKind(x) => x.abstract_city_object_mut(),
        }
    }
}
crate::impl_abstract_city_object_traits!(AbstractCityObjectKindRefMut<'_>);
crate::impl_abstract_city_object_mut_traits!(AbstractCityObjectKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractCityObjectKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractSpaceKind(x) => x.feature_type(),
            Self::AbstractSpaceBoundaryKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_city_object_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::core::refs::AbstractCityObjectKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::AbstractCityObjectKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_with_lifespan_kind_ref_mut!(AbstractCityObjectKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_city_object_kind_ref_mut!($variant, $crate::model::core::$variant);
    };
}
impl_from_for_city_object_kind_ref_mut!(AbstractSpaceKind);
impl_from_for_city_object_kind_ref_mut!(AbstractSpaceBoundaryKind);

#[macro_export]
macro_rules! impl_try_from_for_city_object_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractCityObjectKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractCityObjectKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractCityObjectKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_with_lifespan_kind_ref_mut!(
            AbstractCityObjectKind,
            $type
        );
    };
}

#[macro_export]
macro_rules! impl_try_from_city_object_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractCityObjectKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractCityObjectKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractCityObjectKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_feature_with_lifespan_kind_ref_mut_for_enum!(
            AbstractCityObjectKind,
            $EnumRefMut
        );
    };
}
impl_try_from_feature_with_lifespan_kind_ref_mut_for_enum!(
    AbstractCityObjectKind,
    AbstractCityObjectKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractCityObjectKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::AbstractSpaceKind(x) => x.recompute_bounding_shape(),
            Self::AbstractSpaceBoundaryKind(x) => x.recompute_bounding_shape(),
        }
    }
}
