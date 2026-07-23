use crate::impl_try_from_feature_kind_ref_mut_for_enum;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::{
    AbstractPointCloud, AbstractPointCloudKind, AsAbstractPointCloud, AsAbstractPointCloudMut,
};
use crate::model::point_cloud::PointCloud;
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractPointCloudKindRefMut<'a> {
    PointCloud(&'a mut PointCloud),
}

impl<'a> From<&'a mut AbstractPointCloudKind> for AbstractPointCloudKindRefMut<'a> {
    fn from(item: &'a mut AbstractPointCloudKind) -> Self {
        match item {
            AbstractPointCloudKind::PointCloud(x) => Self::PointCloud(x),
        }
    }
}

impl<'a> AsAbstractPointCloud for AbstractPointCloudKindRefMut<'a> {
    fn abstract_point_cloud(&self) -> &AbstractPointCloud {
        match self {
            Self::PointCloud(x) => x.abstract_point_cloud(),
        }
    }
}

impl<'a> AsAbstractPointCloudMut for AbstractPointCloudKindRefMut<'a> {
    fn abstract_point_cloud_mut(&mut self) -> &mut AbstractPointCloud {
        match self {
            Self::PointCloud(x) => x.abstract_point_cloud_mut(),
        }
    }
}
crate::impl_abstract_point_cloud_traits!(AbstractPointCloudKindRefMut<'_>);
crate::impl_abstract_point_cloud_mut_traits!(AbstractPointCloudKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractPointCloudKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::PointCloud(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_point_cloud_kind_ref_mut {
    ($type:ident) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::core::refs::AbstractPointCloudKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::AbstractPointCloudKindRefMut::$type(x.into())
            }
        }
        $crate::impl_from_for_feature_kind_ref_mut!(AbstractPointCloudKind, $type);
    };
}
impl_from_point_cloud_kind_ref_mut!(PointCloud);

#[macro_export]
macro_rules! impl_try_from_point_cloud_kind_ref_mut {
    ($type:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractPointCloudKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractPointCloudKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractPointCloudKindRefMut::$type(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_kind_ref_mut!(AbstractPointCloudKind, $type);
    };
}
impl_try_from_point_cloud_kind_ref_mut!(PointCloud);
impl_try_from_feature_kind_ref_mut_for_enum!(AbstractPointCloudKind, AbstractPointCloudKindRefMut);

impl<'a> RecomputeBoundingShape for AbstractPointCloudKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::PointCloud(x) => x.recompute_bounding_shape(),
        }
    }
}
