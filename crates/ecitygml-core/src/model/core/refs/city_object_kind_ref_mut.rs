use crate::impl_try_from_feature_with_lifespan_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::{SpaceBoundaryKindRefMut, SpaceKindRefMut};
use crate::model::core::{
    AbstractCityObject, AsAbstractCityObject, AsAbstractCityObjectMut, CityObjectKind,
};

#[derive(Debug)]
pub enum CityObjectKindRefMut<'a> {
    SpaceKind(SpaceKindRefMut<'a>),
    SpaceBoundaryKind(SpaceBoundaryKindRefMut<'a>),
}

impl<'a> From<&'a mut CityObjectKind> for CityObjectKindRefMut<'a> {
    fn from(item: &'a mut CityObjectKind) -> Self {
        match item {
            CityObjectKind::SpaceKind(x) => Self::SpaceKind(x.into()),
            CityObjectKind::SpaceBoundaryKind(x) => Self::SpaceBoundaryKind(x.into()),
        }
    }
}

impl<'a> AsAbstractCityObject for CityObjectKindRefMut<'a> {
    fn abstract_city_object(&self) -> &AbstractCityObject {
        match self {
            Self::SpaceKind(x) => x.abstract_city_object(),
            Self::SpaceBoundaryKind(x) => x.abstract_city_object(),
        }
    }
}

impl<'a> AsAbstractCityObjectMut for CityObjectKindRefMut<'a> {
    fn abstract_city_object_mut(&mut self) -> &mut AbstractCityObject {
        match self {
            Self::SpaceKind(x) => x.abstract_city_object_mut(),
            Self::SpaceBoundaryKind(x) => x.abstract_city_object_mut(),
        }
    }
}
crate::impl_abstract_city_object_traits!(CityObjectKindRefMut<'_>);
crate::impl_abstract_city_object_mut_traits!(CityObjectKindRefMut<'_>);

impl<'a> CityObjectKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::SpaceKind(x) => x.recompute_bounding_shape(),
            Self::SpaceBoundaryKind(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for CityObjectKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::SpaceKind(x) => x.feature_type(),
            Self::SpaceBoundaryKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_city_object_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type> for $crate::model::core::refs::CityObjectKindRefMut<'a> {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::CityObjectKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_with_lifespan_kind_ref_mut!(CityObjectKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_city_object_kind_ref_mut!($variant, $crate::model::core::$variant);
    };
}
impl_from_for_city_object_kind_ref_mut!(SpaceKind);
impl_from_for_city_object_kind_ref_mut!(SpaceBoundaryKind);

#[macro_export]
macro_rules! impl_try_from_for_city_object_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::CityObjectKindRefMut<'a>> for &'a mut $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::CityObjectKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::CityObjectKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_with_lifespan_kind_ref_mut!(CityObjectKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_city_object_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::CityObjectKindRefMut<'a>> for $EnumRefMut<'a> {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::CityObjectKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::CityObjectKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_feature_with_lifespan_kind_ref_mut_for_enum!(
            CityObjectKind,
            $EnumRefMut
        );
    };
}
impl_try_from_feature_with_lifespan_kind_ref_mut_for_enum!(CityObjectKind, CityObjectKindRefMut);
