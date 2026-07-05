use crate::model::core::AsAbstractFeatureMut;
use crate::model::core::refs::{FeatureKindRef, FeatureKindRefMut};
use crate::model::core::{AbstractPointCloud, AsAbstractPointCloud, AsAbstractPointCloudMut};
use egml::model::base::Id;
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use egml::model::geometry::aggregates::MultiPointProperty;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct PointCloud {
    pub(crate) abstract_point_cloud: AbstractPointCloud,
    mime_type: Option<Code>,
    point_file: Option<String>,
    point_file_srs_name: Option<String>,
    points: Option<MultiPointProperty>,
}

impl PointCloud {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_point_cloud(AbstractPointCloud::new(id))
    }

    pub fn from_abstract_point_cloud(abstract_point_cloud: AbstractPointCloud) -> Self {
        Self {
            abstract_point_cloud,
            mime_type: None,
            point_file: None,
            point_file_srs_name: None,
            points: None,
        }
    }

    pub fn mime_type(&self) -> Option<&Code> {
        self.mime_type.as_ref()
    }

    pub fn set_mime_type(&mut self, mime_type: Option<Code>) {
        self.mime_type = mime_type;
    }

    pub fn point_file(&self) -> Option<&str> {
        self.point_file.as_deref()
    }

    pub fn set_point_file(&mut self, point_file: Option<String>) {
        self.point_file = point_file;
    }

    pub fn point_file_srs_name(&self) -> Option<&str> {
        self.point_file_srs_name.as_deref()
    }

    pub fn set_point_file_srs_name(&mut self, point_file_srs_name: Option<String>) {
        self.point_file_srs_name = point_file_srs_name;
    }

    pub fn points(&self) -> Option<&MultiPointProperty> {
        self.points.as_ref()
    }

    pub fn set_points(&mut self, points: Option<MultiPointProperty>) {
        self.points = points;
    }
}

impl PointCloud {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_point_cloud.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_point_cloud.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_point_cloud.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_point_cloud.apply_transform(m);
    }
}

impl AsAbstractPointCloud for PointCloud {
    fn abstract_point_cloud(&self) -> &AbstractPointCloud {
        &self.abstract_point_cloud
    }
}

impl AsAbstractPointCloudMut for PointCloud {
    fn abstract_point_cloud_mut(&mut self) -> &mut AbstractPointCloud {
        &mut self.abstract_point_cloud
    }
}

crate::impl_abstract_point_cloud_traits!(PointCloud);
crate::impl_abstract_point_cloud_mut_traits!(PointCloud);
crate::impl_has_feature_type!(PointCloud, PointCloud);
