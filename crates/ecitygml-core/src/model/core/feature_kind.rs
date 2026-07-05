use crate::model::appearance::SurfaceDataKind;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::feature_with_lifespan_kind::FeatureWithLifespanKind;
use crate::model::core::{
    AbstractFeature, AsAbstractFeature, AsAbstractFeatureMut, PointCloudKind,
};

#[derive(Debug, Clone, PartialEq)]
pub enum FeatureKind {
    FeatureWithLifespanKind(FeatureWithLifespanKind),
    SurfaceDataKind(SurfaceDataKind),
    PointCloudKind(PointCloudKind),
}

impl AsAbstractFeature for FeatureKind {
    fn abstract_feature(&self) -> &AbstractFeature {
        match self {
            FeatureKind::FeatureWithLifespanKind(x) => x.abstract_feature(),
            FeatureKind::SurfaceDataKind(x) => x.abstract_feature(),
            FeatureKind::PointCloudKind(x) => x.abstract_feature(),
        }
    }
}

impl AsAbstractFeatureMut for FeatureKind {
    fn abstract_feature_mut(&mut self) -> &mut AbstractFeature {
        match self {
            FeatureKind::FeatureWithLifespanKind(x) => x.abstract_feature_mut(),
            FeatureKind::SurfaceDataKind(x) => x.abstract_feature_mut(),
            FeatureKind::PointCloudKind(x) => x.abstract_feature_mut(),
        }
    }
}

impl HasFeatureType for FeatureKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::FeatureWithLifespanKind(x) => x.feature_type(),
            Self::SurfaceDataKind(x) => x.feature_type(),
            Self::PointCloudKind(x) => x.feature_type(),
        }
    }
}

// impl_abstract_feature_traits!(FeatureKind);

#[macro_export]
macro_rules! impl_from_for_feature_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::FeatureKind {
            fn from(x: $type) -> Self {
                $crate::model::core::FeatureKind::$variant(x.into())
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_from_for_feature_kind!($variant, $variant);
    };
}
impl_from_for_feature_kind!(FeatureWithLifespanKind);
impl_from_for_feature_kind!(SurfaceDataKind);

#[macro_export]
macro_rules! impl_try_from_for_feature_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::FeatureKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::FeatureKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::FeatureKind::$variant(k) => k.try_into().map_err(|_| ()),
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
