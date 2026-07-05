use crate::model::appearance::SurfaceDataKind;
use crate::model::appearance::refs::SurfaceDataKindRefMut;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::{FeatureWithLifespanKindRefMut, PointCloudKindRefMut};
use crate::model::core::{
    AbstractFeature, AsAbstractFeature, AsAbstractFeatureMut, FeatureKind, FeatureWithLifespanKind,
    PointCloudKind,
};

#[derive(Debug)]
pub enum FeatureKindRefMut<'a> {
    FeatureWithLifespanKind(FeatureWithLifespanKindRefMut<'a>),
    SurfaceDataKind(SurfaceDataKindRefMut<'a>),
    PointCloudKind(PointCloudKindRefMut<'a>),
}

impl<'a> From<&'a mut FeatureKind> for FeatureKindRefMut<'a> {
    fn from(item: &'a mut FeatureKind) -> Self {
        match item {
            FeatureKind::FeatureWithLifespanKind(x) => Self::FeatureWithLifespanKind(x.into()),
            FeatureKind::SurfaceDataKind(x) => Self::SurfaceDataKind(x.into()),
            FeatureKind::PointCloudKind(x) => Self::PointCloudKind(x.into()),
        }
    }
}

impl<'a> AsAbstractFeature for FeatureKindRefMut<'a> {
    fn abstract_feature(&self) -> &AbstractFeature {
        match self {
            Self::FeatureWithLifespanKind(x) => x.abstract_feature(),
            Self::SurfaceDataKind(x) => x.abstract_feature(),
            Self::PointCloudKind(x) => x.abstract_feature(),
        }
    }
}

impl<'a> AsAbstractFeatureMut for FeatureKindRefMut<'a> {
    fn abstract_feature_mut(&mut self) -> &mut AbstractFeature {
        match self {
            Self::FeatureWithLifespanKind(x) => x.abstract_feature_mut(),
            Self::SurfaceDataKind(x) => x.abstract_feature_mut(),
            Self::PointCloudKind(x) => x.abstract_feature_mut(),
        }
    }
}

impl<'a> FeatureKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::FeatureWithLifespanKind(x) => x.recompute_bounding_shape(),
            Self::SurfaceDataKind(x) => x.recompute_bounding_shape(),
            Self::PointCloudKind(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for FeatureKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::FeatureWithLifespanKind(x) => x.feature_type(),
            Self::SurfaceDataKind(x) => x.feature_type(),
            Self::PointCloudKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_feature_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type> for $crate::model::core::refs::FeatureKindRefMut<'a> {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::FeatureKindRefMut::$variant(x.into())
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_from_for_feature_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_feature_kind_ref_mut!(FeatureWithLifespanKind);
impl_from_for_feature_kind_ref_mut!(SurfaceDataKind);
impl_from_for_feature_kind_ref_mut!(PointCloudKind);

#[macro_export]
macro_rules! impl_try_from_for_feature_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::FeatureKindRefMut<'a>> for &'a mut $type {
            type Error = ();
            fn try_from(x: $crate::model::core::refs::FeatureKindRefMut<'a>) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::FeatureKindRefMut::$variant(k) => {
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
        impl<'a> TryFrom<$crate::model::core::refs::FeatureKindRefMut<'a>> for $EnumRefMut<'a> {
            type Error = ();
            fn try_from(x: $crate::model::core::refs::FeatureKindRefMut<'a>) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::FeatureKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
}
