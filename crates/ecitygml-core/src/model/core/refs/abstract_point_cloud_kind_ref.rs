use crate::impl_try_from_feature_kind_ref_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::{AbstractPointCloud, AbstractPointCloudKind, AsAbstractPointCloud};
use crate::model::point_cloud::PointCloud;

#[derive(Debug, Clone, Copy)]
pub enum AbstractPointCloudKindRef<'a> {
    PointCloud(&'a PointCloud),
}

impl<'a> From<&'a AbstractPointCloudKind> for AbstractPointCloudKindRef<'a> {
    fn from(item: &'a AbstractPointCloudKind) -> Self {
        match item {
            AbstractPointCloudKind::PointCloud(x) => Self::PointCloud(x),
        }
    }
}

impl<'a> AsAbstractPointCloud for AbstractPointCloudKindRef<'a> {
    fn abstract_point_cloud(&self) -> &AbstractPointCloud {
        match self {
            Self::PointCloud(x) => x.abstract_point_cloud(),
        }
    }
}
crate::impl_abstract_point_cloud_traits!(AbstractPointCloudKindRef<'_>);

impl<'a> HasFeatureType for AbstractPointCloudKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::PointCloud(_) => FeatureType::PointCloud,
        }
    }
}

#[macro_export]
macro_rules! impl_from_point_cloud_kind_ref {
    ($type:ident) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::AbstractPointCloudKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::AbstractPointCloudKindRef::$type(x.into())
            }
        }
        $crate::impl_from_for_feature_kind_ref!(AbstractPointCloudKind, $type);
    };
}
impl_from_point_cloud_kind_ref!(PointCloud);

#[macro_export]
macro_rules! impl_try_from_point_cloud_kind_ref {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractPointCloudKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractPointCloudKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractPointCloudKindRef::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_kind_ref!(AbstractPointCloudKind, $type);
    };
}
impl_try_from_point_cloud_kind_ref!(PointCloud);
impl_try_from_feature_kind_ref_for_enum!(AbstractPointCloudKind, AbstractPointCloudKindRef);
