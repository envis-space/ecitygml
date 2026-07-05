use crate::model::appearance::SurfaceDataKind;
use crate::model::appearance::refs::SurfaceDataKindRef;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::{FeatureWithLifespanKindRef, PointCloudKindRef};
use crate::model::core::{
    AbstractFeature, AsAbstractFeature, FeatureKind, FeatureWithLifespanKind, PointCloudKind,
};

#[derive(Debug, Clone, Copy)]
pub enum FeatureKindRef<'a> {
    FeatureWithLifespanKind(FeatureWithLifespanKindRef<'a>),
    SurfaceDataKind(SurfaceDataKindRef<'a>),
    PointCloudKind(PointCloudKindRef<'a>),
}

impl<'a> From<&'a FeatureKind> for FeatureKindRef<'a> {
    fn from(item: &'a FeatureKind) -> Self {
        match item {
            FeatureKind::FeatureWithLifespanKind(x) => Self::FeatureWithLifespanKind(x.into()),
            FeatureKind::SurfaceDataKind(x) => Self::SurfaceDataKind(x.into()),
            FeatureKind::PointCloudKind(x) => Self::PointCloudKind(x.into()),
        }
    }
}

impl<'a> AsAbstractFeature for FeatureKindRef<'a> {
    fn abstract_feature(&self) -> &AbstractFeature {
        match self {
            Self::FeatureWithLifespanKind(x) => x.abstract_feature(),
            Self::SurfaceDataKind(x) => x.abstract_feature(),
            Self::PointCloudKind(x) => x.abstract_feature(),
        }
    }
}

impl<'a> HasFeatureType for FeatureKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::FeatureWithLifespanKind(x) => x.feature_type(),
            Self::SurfaceDataKind(x) => x.feature_type(),
            Self::PointCloudKind(x) => x.feature_type(),
        }
    }
}
#[macro_export]
macro_rules! impl_from_for_feature_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::FeatureKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::FeatureKindRef::$variant(x.into())
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_from_for_feature_kind_ref!($variant, $variant);
    };
}
impl_from_for_feature_kind_ref!(FeatureWithLifespanKind);
impl_from_for_feature_kind_ref!(SurfaceDataKind);
impl_from_for_feature_kind_ref!(PointCloudKind);

#[macro_export]
macro_rules! impl_try_from_for_feature_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::FeatureKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(x: $crate::model::core::refs::FeatureKindRef<'a>) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::FeatureKindRef::$variant(k) => {
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
macro_rules! impl_try_from_feature_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::FeatureKindRef<'a>> for $EnumRef<'a> {
            type Error = ();
            fn try_from(x: $crate::model::core::refs::FeatureKindRef<'a>) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::FeatureKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
}
