use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::refs::{FeatureKindRef, FeatureKindRefMut};
use crate::model::core::{AbstractPointCloud, AsAbstractPointCloud, AsAbstractPointCloudMut};
use crate::model::point_cloud::PointCloud;
use crate::{impl_abstract_point_cloud_mut_traits, impl_abstract_point_cloud_traits};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum PointCloudKind {
    PointCloud(PointCloud),
}

impl PointCloudKind {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            PointCloudKind::PointCloud(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            PointCloudKind::PointCloud(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            PointCloudKind::PointCloud(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            PointCloudKind::PointCloud(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            PointCloudKind::PointCloud(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractPointCloud for PointCloudKind {
    fn abstract_point_cloud(&self) -> &AbstractPointCloud {
        match self {
            PointCloudKind::PointCloud(x) => x.abstract_point_cloud(),
        }
    }
}

impl AsAbstractPointCloudMut for PointCloudKind {
    fn abstract_point_cloud_mut(&mut self) -> &mut AbstractPointCloud {
        match self {
            PointCloudKind::PointCloud(x) => x.abstract_point_cloud_mut(),
        }
    }
}

impl_abstract_point_cloud_traits!(PointCloudKind);
impl_abstract_point_cloud_mut_traits!(PointCloudKind);

impl HasFeatureType for PointCloudKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::PointCloud(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_point_cloud_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::PointCloudKind {
            fn from(x: $type) -> Self {
                $crate::model::core::PointCloudKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_kind!(PointCloudKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_point_cloud_kind!($variant, $variant);
    };
}
impl_from_for_point_cloud_kind!(PointCloud);

#[macro_export]
macro_rules! impl_try_from_for_point_cloud_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::PointCloudKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::PointCloudKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::PointCloudKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_kind!(PointCloudKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_point_cloud_kind!($variant, $variant);
    };
}
impl_try_from_for_point_cloud_kind!(PointCloud);
