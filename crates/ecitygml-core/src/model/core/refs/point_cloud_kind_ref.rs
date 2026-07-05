use crate::impl_try_from_feature_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::{AbstractPointCloud, AsAbstractPointCloud, PointCloudKind};
use crate::model::point_cloud::PointCloud;

#[derive(Debug, Clone, Copy)]
pub enum PointCloudKindRef<'a> {
    PointCloud(&'a PointCloud),
}

impl<'a> From<&'a PointCloudKind> for PointCloudKindRef<'a> {
    fn from(item: &'a PointCloudKind) -> Self {
        match item {
            PointCloudKind::PointCloud(x) => Self::PointCloud(x),
        }
    }
}

impl<'a> AsAbstractPointCloud for PointCloudKindRef<'a> {
    fn abstract_point_cloud(&self) -> &AbstractPointCloud {
        match self {
            Self::PointCloud(x) => x.abstract_point_cloud(),
        }
    }
}
crate::impl_abstract_point_cloud_traits!(PointCloudKindRef<'_>);

impl<'a> HasFeatureType for PointCloudKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::PointCloud(_) => FeatureType::PointCloud,
        }
    }
}

#[macro_export]
macro_rules! impl_from_point_cloud_kind_ref {
    ($type:ident) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::PointCloudKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::PointCloudKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_feature_kind_ref!(PointCloudKind, $type);
    };
}
impl_from_point_cloud_kind_ref!(PointCloud);

#[macro_export]
macro_rules! impl_try_from_point_cloud_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::PointCloudKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(x: $crate::model::core::refs::PointCloudKindRef<'a>) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::PointCloudKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_kind_ref!(PointCloudKind, $type);
    };
}
impl_try_from_point_cloud_kind_ref!(PointCloud);
impl_try_from_feature_kind_ref_for_enum!(PointCloudKind, PointCloudKindRef);
