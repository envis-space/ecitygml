use crate::model::appearance::AbstractSurfaceDataKind;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::abstract_feature_with_lifespan_kind::AbstractFeatureWithLifespanKind;
use crate::model::core::{
    AbstractFeature, AbstractPointCloudKind, AsAbstractFeature, AsAbstractFeatureMut,
};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractFeatureKind {
    AbstractFeatureWithLifespanKind(AbstractFeatureWithLifespanKind),
    AbstractSurfaceDataKind(AbstractSurfaceDataKind),
    AbstractPointCloudKind(AbstractPointCloudKind),
}

impl AsAbstractFeature for AbstractFeatureKind {
    fn abstract_feature(&self) -> &AbstractFeature {
        match self {
            AbstractFeatureKind::AbstractFeatureWithLifespanKind(x) => x.abstract_feature(),
            AbstractFeatureKind::AbstractSurfaceDataKind(x) => x.abstract_feature(),
            AbstractFeatureKind::AbstractPointCloudKind(x) => x.abstract_feature(),
        }
    }
}

impl AsAbstractFeatureMut for AbstractFeatureKind {
    fn abstract_feature_mut(&mut self) -> &mut AbstractFeature {
        match self {
            AbstractFeatureKind::AbstractFeatureWithLifespanKind(x) => x.abstract_feature_mut(),
            AbstractFeatureKind::AbstractSurfaceDataKind(x) => x.abstract_feature_mut(),
            AbstractFeatureKind::AbstractPointCloudKind(x) => x.abstract_feature_mut(),
        }
    }
}

crate::impl_abstract_feature_traits!(AbstractFeatureKind);
crate::impl_abstract_feature_mut_traits!(AbstractFeatureKind);

impl HasFeatureType for AbstractFeatureKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractFeatureWithLifespanKind(x) => x.feature_type(),
            Self::AbstractSurfaceDataKind(x) => x.feature_type(),
            Self::AbstractPointCloudKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_feature_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::AbstractFeatureKind {
            fn from(x: $type) -> Self {
                $crate::model::core::AbstractFeatureKind::$variant(x.into())
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_from_for_feature_kind!($variant, $variant);
    };
}
impl_from_for_feature_kind!(AbstractFeatureWithLifespanKind);
impl_from_for_feature_kind!(AbstractSurfaceDataKind);
impl_from_for_feature_kind!(AbstractPointCloudKind);

#[macro_export]
macro_rules! impl_try_from_for_feature_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::AbstractFeatureKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::AbstractFeatureKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::AbstractFeatureKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_feature_kind!($variant, $variant);
    };
}
