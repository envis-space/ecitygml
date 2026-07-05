use crate::impl_try_from_feature_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::{
    AbstractPointCloud, AsAbstractPointCloud, AsAbstractPointCloudMut, PointCloudKind,
};
use crate::model::point_cloud::PointCloud;

#[derive(Debug)]
pub enum PointCloudKindRefMut<'a> {
    PointCloud(&'a mut PointCloud),
}

impl<'a> From<&'a mut PointCloudKind> for PointCloudKindRefMut<'a> {
    fn from(item: &'a mut PointCloudKind) -> Self {
        match item {
            PointCloudKind::PointCloud(x) => Self::PointCloud(x),
        }
    }
}

impl<'a> AsAbstractPointCloud for PointCloudKindRefMut<'a> {
    fn abstract_point_cloud(&self) -> &AbstractPointCloud {
        match self {
            Self::PointCloud(x) => x.abstract_point_cloud(),
        }
    }
}

impl<'a> AsAbstractPointCloudMut for PointCloudKindRefMut<'a> {
    fn abstract_point_cloud_mut(&mut self) -> &mut AbstractPointCloud {
        match self {
            Self::PointCloud(x) => x.abstract_point_cloud_mut(),
        }
    }
}
crate::impl_abstract_point_cloud_traits!(PointCloudKindRefMut<'_>);
crate::impl_abstract_point_cloud_mut_traits!(PointCloudKindRefMut<'_>);

impl<'a> PointCloudKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::PointCloud(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for PointCloudKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::PointCloud(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_point_cloud_kind_ref_mut {
    ($type:ident) => {
        impl<'a> From<&'a mut $type> for $crate::model::core::refs::PointCloudKindRefMut<'a> {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::PointCloudKindRefMut::$type(x.into())
            }
        }
        $crate::impl_from_for_feature_kind_ref_mut!(PointCloudKind, $type);
    };
}
impl_from_point_cloud_kind_ref_mut!(PointCloud);

#[macro_export]
macro_rules! impl_try_from_point_cloud_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::PointCloudKindRefMut<'a>> for &'a mut $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::PointCloudKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::PointCloudKindRefMut::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_kind_ref_mut!(PointCloudKind, $type);
    };
}
impl_try_from_point_cloud_kind_ref_mut!(PointCloud);
impl_try_from_feature_kind_ref_mut_for_enum!(PointCloudKind, PointCloudKindRefMut);
