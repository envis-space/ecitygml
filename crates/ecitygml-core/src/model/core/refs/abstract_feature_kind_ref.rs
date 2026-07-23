use crate::model::appearance::AbstractSurfaceDataKind;
use crate::model::appearance::refs::AbstractSurfaceDataKindRef;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::refs::{AbstractFeatureWithLifespanKindRef, AbstractPointCloudKindRef};
use crate::model::core::{
    AbstractFeature, AbstractFeatureKind, AbstractFeatureWithLifespanKind, AbstractPointCloudKind,
    AsAbstractFeature,
};

#[derive(Debug, Clone, Copy)]
pub enum AbstractFeatureKindRef<'a> {
    AbstractFeatureWithLifespanKind(AbstractFeatureWithLifespanKindRef<'a>),
    AbstractSurfaceDataKind(AbstractSurfaceDataKindRef<'a>),
    AbstractPointCloudKind(AbstractPointCloudKindRef<'a>),
}

impl<'a> From<&'a AbstractFeatureKind> for AbstractFeatureKindRef<'a> {
    fn from(item: &'a AbstractFeatureKind) -> Self {
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

impl<'a> AsAbstractFeature for AbstractFeatureKindRef<'a> {
    fn abstract_feature(&self) -> &AbstractFeature {
        match self {
            Self::AbstractFeatureWithLifespanKind(x) => x.abstract_feature(),
            Self::AbstractSurfaceDataKind(x) => x.abstract_feature(),
            Self::AbstractPointCloudKind(x) => x.abstract_feature(),
        }
    }
}
crate::impl_abstract_feature_traits!(AbstractFeatureKindRef<'_>);

impl<'a> HasFeatureType for AbstractFeatureKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::AbstractFeatureWithLifespanKind(x) => x.feature_type(),
            Self::AbstractSurfaceDataKind(x) => x.feature_type(),
            Self::AbstractPointCloudKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_feature_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::AbstractFeatureKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::AbstractFeatureKindRef::$variant(x.into())
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_from_for_feature_kind_ref!($variant, $variant);
    };
}
impl_from_for_feature_kind_ref!(AbstractFeatureWithLifespanKind);
impl_from_for_feature_kind_ref!(AbstractSurfaceDataKind);
impl_from_for_feature_kind_ref!(AbstractPointCloudKind);

#[macro_export]
macro_rules! impl_try_from_for_feature_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractFeatureKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractFeatureKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractFeatureKindRef::$variant(k) => {
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
        impl<'a> TryFrom<$crate::model::core::refs::AbstractFeatureKindRef<'a>> for $EnumRef<'a> {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractFeatureKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractFeatureKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
}
