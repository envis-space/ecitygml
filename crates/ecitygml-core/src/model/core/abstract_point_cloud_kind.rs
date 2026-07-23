use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::refs::{AbstractFeatureKindRef, AbstractFeatureKindRefMut};
use crate::model::core::{AbstractPointCloud, AsAbstractPointCloud, AsAbstractPointCloudMut};
use crate::model::point_cloud::PointCloud;
use crate::{impl_abstract_point_cloud_mut_traits, impl_abstract_point_cloud_traits};
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractPointCloudKind {
    PointCloud(PointCloud),
}

impl AsAbstractPointCloud for AbstractPointCloudKind {
    fn abstract_point_cloud(&self) -> &AbstractPointCloud {
        match self {
            AbstractPointCloudKind::PointCloud(x) => x.abstract_point_cloud(),
        }
    }
}

impl AsAbstractPointCloudMut for AbstractPointCloudKind {
    fn abstract_point_cloud_mut(&mut self) -> &mut AbstractPointCloud {
        match self {
            AbstractPointCloudKind::PointCloud(x) => x.abstract_point_cloud_mut(),
        }
    }
}

impl_abstract_point_cloud_traits!(AbstractPointCloudKind);
impl_abstract_point_cloud_mut_traits!(AbstractPointCloudKind);

impl HasFeatureType for AbstractPointCloudKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::PointCloud(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_point_cloud_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::AbstractPointCloudKind {
            fn from(x: $type) -> Self {
                $crate::model::core::AbstractPointCloudKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_kind!(AbstractPointCloudKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_point_cloud_kind!($variant, $variant);
    };
}
impl_from_for_point_cloud_kind!(PointCloud);

#[macro_export]
macro_rules! impl_try_from_for_point_cloud_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::AbstractPointCloudKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::AbstractPointCloudKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::AbstractPointCloudKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_kind!(AbstractPointCloudKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_point_cloud_kind!($variant, $variant);
    };
}
impl_try_from_for_point_cloud_kind!(PointCloud);

impl IterFeatures for AbstractPointCloudKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractPointCloudKind::PointCloud(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractPointCloudKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractPointCloudKind::PointCloud(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractPointCloudKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractPointCloudKind::PointCloud(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractPointCloudKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractPointCloudKind::PointCloud(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractPointCloudKind::PointCloud(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractPointCloudKind::PointCloud(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractPointCloudKind::PointCloud(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractPointCloudKind::PointCloud(x) => x.apply_scale(scale),
        }
    }
}
