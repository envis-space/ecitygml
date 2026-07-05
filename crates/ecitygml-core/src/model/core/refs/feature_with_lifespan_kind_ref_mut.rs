use crate::impl_try_from_feature_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::AppearanceKind;
use crate::model::core::CityObjectKind;
use crate::model::core::refs::{AppearanceKindRefMut, CityObjectKindRefMut};
use crate::model::core::{
    AbstractFeatureWithLifespan, AsAbstractFeatureWithLifespan, AsAbstractFeatureWithLifespanMut,
    CityModel, FeatureWithLifespanKind,
};

#[derive(Debug)]
pub enum FeatureWithLifespanKindRefMut<'a> {
    AppearanceKind(AppearanceKindRefMut<'a>),
    CityModel(&'a mut CityModel),
    CityObjectKind(CityObjectKindRefMut<'a>),
}

impl<'a> From<&'a mut FeatureWithLifespanKind> for FeatureWithLifespanKindRefMut<'a> {
    fn from(item: &'a mut FeatureWithLifespanKind) -> Self {
        match item {
            FeatureWithLifespanKind::AppearanceKind(x) => Self::AppearanceKind(x.into()),
            FeatureWithLifespanKind::CityModel(x) => Self::CityModel(x),
            FeatureWithLifespanKind::CityObjectKind(x) => Self::CityObjectKind(x.into()),
        }
    }
}

impl<'a> AsAbstractFeatureWithLifespan for FeatureWithLifespanKindRefMut<'a> {
    fn abstract_feature_with_lifespan(&self) -> &AbstractFeatureWithLifespan {
        match self {
            Self::AppearanceKind(x) => x.abstract_feature_with_lifespan(),
            Self::CityModel(x) => x.abstract_feature_with_lifespan(),
            Self::CityObjectKind(x) => x.abstract_feature_with_lifespan(),
        }
    }
}

impl<'a> AsAbstractFeatureWithLifespanMut for FeatureWithLifespanKindRefMut<'a> {
    fn abstract_feature_with_lifespan_mut(&mut self) -> &mut AbstractFeatureWithLifespan {
        match self {
            Self::AppearanceKind(x) => x.abstract_feature_with_lifespan_mut(),
            Self::CityModel(x) => x.abstract_feature_with_lifespan_mut(),
            Self::CityObjectKind(x) => x.abstract_feature_with_lifespan_mut(),
        }
    }
}
crate::impl_abstract_feature_with_lifespan_traits!(FeatureWithLifespanKindRefMut<'_>);
crate::impl_abstract_feature_with_lifespan_mut_traits!(FeatureWithLifespanKindRefMut<'_>);

impl<'a> FeatureWithLifespanKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::AppearanceKind(x) => x.recompute_bounding_shape(),
            Self::CityModel(_) => {}
            Self::CityObjectKind(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for FeatureWithLifespanKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AppearanceKind(x) => x.feature_type(),
            Self::CityModel(_) => unreachable!(),
            Self::CityObjectKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_feature_with_lifespan_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::core::refs::FeatureWithLifespanKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::FeatureWithLifespanKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_kind_ref_mut!(FeatureWithLifespanKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_feature_with_lifespan_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_feature_with_lifespan_kind_ref_mut!(AppearanceKind);
impl_from_for_feature_with_lifespan_kind_ref_mut!(CityModel);
impl_from_for_feature_with_lifespan_kind_ref_mut!(CityObjectKind);

#[macro_export]
macro_rules! impl_try_from_for_feature_with_lifespan_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::FeatureWithLifespanKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::FeatureWithLifespanKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::FeatureWithLifespanKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_kind_ref_mut!(FeatureWithLifespanKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_feature_with_lifespan_kind_ref_mut!($variant, $variant);
    };
}
impl_try_from_for_feature_with_lifespan_kind_ref_mut!(CityModel);

#[macro_export]
macro_rules! impl_try_from_feature_with_lifespan_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::FeatureWithLifespanKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::FeatureWithLifespanKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::FeatureWithLifespanKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_feature_kind_ref_mut_for_enum!(FeatureWithLifespanKind, $EnumRefMut);
    };
}
impl_try_from_feature_kind_ref_mut_for_enum!(
    FeatureWithLifespanKind,
    FeatureWithLifespanKindRefMut
);
