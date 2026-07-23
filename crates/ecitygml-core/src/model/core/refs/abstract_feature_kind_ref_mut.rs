use crate::model::appearance::AbstractSurfaceDataKind;
use crate::model::appearance::refs::AbstractSurfaceDataKindRefMut;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::{
    AbstractFeatureWithLifespanKindRefMut, AbstractPointCloudKindRefMut,
};
use egml::model::common::RecomputeBoundingShape;

use crate::model::core::{
    AbstractFeature, AbstractFeatureKind, AbstractFeatureWithLifespanKind, AbstractPointCloudKind,
    AsAbstractFeature, AsAbstractFeatureMut,
};

#[derive(Debug)]
pub enum AbstractFeatureKindRefMut<'a> {
    AbstractFeatureWithLifespanKind(AbstractFeatureWithLifespanKindRefMut<'a>),
    AbstractSurfaceDataKind(AbstractSurfaceDataKindRefMut<'a>),
    AbstractPointCloudKind(AbstractPointCloudKindRefMut<'a>),
}

impl<'a> From<&'a mut AbstractFeatureKind> for AbstractFeatureKindRefMut<'a> {
    fn from(item: &'a mut AbstractFeatureKind) -> Self {
        match item {
            AbstractFeatureKind::AbstractFeatureWithLifespanKind(x) => {
                Self::AbstractFeatureWithLifespanKind(x.into())
            }
            AbstractFeatureKind::AbstractSurfaceDataKind(x) => {
                Self::AbstractSurfaceDataKind(x.into())
            }
            AbstractFeatureKind::AbstractPointCloudKind(x) => {
                Self::AbstractPointCloudKind(x.into())
            }
        }
    }
}

impl<'a> AsAbstractFeature for AbstractFeatureKindRefMut<'a> {
    fn abstract_feature(&self) -> &AbstractFeature {
        match self {
            Self::AbstractFeatureWithLifespanKind(x) => x.abstract_feature(),
            Self::AbstractSurfaceDataKind(x) => x.abstract_feature(),
            Self::AbstractPointCloudKind(x) => x.abstract_feature(),
        }
    }
}

impl<'a> AsAbstractFeatureMut for AbstractFeatureKindRefMut<'a> {
    fn abstract_feature_mut(&mut self) -> &mut AbstractFeature {
        match self {
            Self::AbstractFeatureWithLifespanKind(x) => x.abstract_feature_mut(),
            Self::AbstractSurfaceDataKind(x) => x.abstract_feature_mut(),
            Self::AbstractPointCloudKind(x) => x.abstract_feature_mut(),
        }
    }
}
crate::impl_abstract_feature_traits!(AbstractFeatureKindRefMut<'_>);
crate::impl_abstract_feature_mut_traits!(AbstractFeatureKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractFeatureKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractFeatureWithLifespanKind(x) => x.feature_type(),
            Self::AbstractSurfaceDataKind(x) => x.feature_type(),
            Self::AbstractPointCloudKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_feature_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type> for $crate::model::core::refs::AbstractFeatureKindRefMut<'a> {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::AbstractFeatureKindRefMut::$variant(x.into())
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_from_for_feature_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_feature_kind_ref_mut!(AbstractFeatureWithLifespanKind);
impl_from_for_feature_kind_ref_mut!(AbstractSurfaceDataKind);
impl_from_for_feature_kind_ref_mut!(AbstractPointCloudKind);

#[macro_export]
macro_rules! impl_try_from_for_feature_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractFeatureKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractFeatureKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractFeatureKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_try_from_feature_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractFeatureKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractFeatureKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractFeatureKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
}

impl<'a> RecomputeBoundingShape for AbstractFeatureKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::AbstractFeatureWithLifespanKind(x) => x.recompute_bounding_shape(),
            Self::AbstractSurfaceDataKind(x) => x.recompute_bounding_shape(),
            Self::AbstractPointCloudKind(x) => x.recompute_bounding_shape(),
        }
    }
}
