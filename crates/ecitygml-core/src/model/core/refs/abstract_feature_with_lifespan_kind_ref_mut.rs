use crate::impl_try_from_feature_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::AbstractAppearanceKind;
use crate::model::core::AbstractCityObjectKind;
use crate::model::core::refs::{AbstractAppearanceKindRefMut, AbstractCityObjectKindRefMut};
use crate::model::core::{
    AbstractFeatureWithLifespan, AbstractFeatureWithLifespanKind, AsAbstractFeatureWithLifespan,
    AsAbstractFeatureWithLifespanMut, CityModel,
};
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractFeatureWithLifespanKindRefMut<'a> {
    AbstractAppearanceKind(AbstractAppearanceKindRefMut<'a>),
    CityModel(&'a mut CityModel),
    AbstractCityObjectKind(AbstractCityObjectKindRefMut<'a>),
}

impl<'a> From<&'a mut AbstractFeatureWithLifespanKind>
    for AbstractFeatureWithLifespanKindRefMut<'a>
{
    fn from(item: &'a mut AbstractFeatureWithLifespanKind) -> Self {
        match item {
            AbstractFeatureWithLifespanKind::AbstractAppearanceKind(x) => {
                Self::AbstractAppearanceKind(x.into())
            }
            AbstractFeatureWithLifespanKind::CityModel(x) => Self::CityModel(x),
            AbstractFeatureWithLifespanKind::AbstractCityObjectKind(x) => {
                Self::AbstractCityObjectKind(x.into())
            }
        }
    }
}

impl<'a> AsAbstractFeatureWithLifespan for AbstractFeatureWithLifespanKindRefMut<'a> {
    fn abstract_feature_with_lifespan(&self) -> &AbstractFeatureWithLifespan {
        match self {
            Self::AbstractAppearanceKind(x) => x.abstract_feature_with_lifespan(),
            Self::CityModel(x) => x.abstract_feature_with_lifespan(),
            Self::AbstractCityObjectKind(x) => x.abstract_feature_with_lifespan(),
        }
    }
}

impl<'a> AsAbstractFeatureWithLifespanMut for AbstractFeatureWithLifespanKindRefMut<'a> {
    fn abstract_feature_with_lifespan_mut(&mut self) -> &mut AbstractFeatureWithLifespan {
        match self {
            Self::AbstractAppearanceKind(x) => x.abstract_feature_with_lifespan_mut(),
            Self::CityModel(x) => x.abstract_feature_with_lifespan_mut(),
            Self::AbstractCityObjectKind(x) => x.abstract_feature_with_lifespan_mut(),
        }
    }
}
crate::impl_abstract_feature_with_lifespan_traits!(AbstractFeatureWithLifespanKindRefMut<'_>);
crate::impl_abstract_feature_with_lifespan_mut_traits!(AbstractFeatureWithLifespanKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractFeatureWithLifespanKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractAppearanceKind(x) => x.feature_type(),
            Self::CityModel(_) => FeatureType::CityModel,
            Self::AbstractCityObjectKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_feature_with_lifespan_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::core::refs::AbstractFeatureWithLifespanKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::AbstractFeatureWithLifespanKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_kind_ref_mut!(AbstractFeatureWithLifespanKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_feature_with_lifespan_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_feature_with_lifespan_kind_ref_mut!(AbstractAppearanceKind);
impl_from_for_feature_with_lifespan_kind_ref_mut!(CityModel);
impl_from_for_feature_with_lifespan_kind_ref_mut!(AbstractCityObjectKind);

#[macro_export]
macro_rules! impl_try_from_for_feature_with_lifespan_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractFeatureWithLifespanKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractFeatureWithLifespanKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractFeatureWithLifespanKindRefMut::$variant(
                        k,
                    ) => k.try_into().map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_kind_ref_mut!(AbstractFeatureWithLifespanKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_feature_with_lifespan_kind_ref_mut!($variant, $variant);
    };
}
impl_try_from_for_feature_with_lifespan_kind_ref_mut!(CityModel);

#[macro_export]
macro_rules! impl_try_from_feature_with_lifespan_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractFeatureWithLifespanKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractFeatureWithLifespanKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractFeatureWithLifespanKindRefMut::$variant(
                        k,
                    ) => $EnumRefMut::try_from(k).map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_feature_kind_ref_mut_for_enum!(
            AbstractFeatureWithLifespanKind,
            $EnumRefMut
        );
    };
}
impl_try_from_feature_kind_ref_mut_for_enum!(
    AbstractFeatureWithLifespanKind,
    AbstractFeatureWithLifespanKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractFeatureWithLifespanKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::AbstractAppearanceKind(x) => x.recompute_bounding_shape(),
            Self::CityModel(_) => {}
            Self::AbstractCityObjectKind(x) => x.recompute_bounding_shape(),
        }
    }
}
