use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::{AbstractFeatureKindRef, AbstractFeatureKindRefMut};
use crate::model::core::values::MimeTypeValue;
use crate::model::core::{AbstractPointCloud, AsAbstractPointCloud, AsAbstractPointCloudMut};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use egml::model::geometry::aggregates::MultiPointProperty;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct PointCloud {
    pub(crate) abstract_point_cloud: AbstractPointCloud,
    mime_type: Option<MimeTypeValue>,
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

    pub fn mime_type(&self) -> Option<&MimeTypeValue> {
        self.mime_type.as_ref()
    }

    pub fn set_mime_type(&mut self, mime_type: MimeTypeValue) {
        self.mime_type = Some(mime_type);
    }

    pub fn set_mime_type_opt(&mut self, mime_type: Option<MimeTypeValue>) {
        self.mime_type = mime_type;
    }

    pub fn clear_mime_type(&mut self) {
        self.mime_type = None;
    }

    pub fn point_file(&self) -> Option<&str> {
        self.point_file.as_deref()
    }

    pub fn set_point_file(&mut self, point_file: String) {
        self.point_file = Some(point_file);
    }

    pub fn set_point_file_opt(&mut self, point_file: Option<String>) {
        self.point_file = point_file;
    }

    pub fn clear_point_file(&mut self) {
        self.point_file = None;
    }

    pub fn point_file_srs_name(&self) -> Option<&str> {
        self.point_file_srs_name.as_deref()
    }

    pub fn set_point_file_srs_name(&mut self, point_file_srs_name: String) {
        self.point_file_srs_name = Some(point_file_srs_name);
    }

    pub fn set_point_file_srs_name_opt(&mut self, point_file_srs_name: Option<String>) {
        self.point_file_srs_name = point_file_srs_name;
    }

    pub fn clear_point_file_srs_name(&mut self) {
        self.point_file_srs_name = None;
    }

    pub fn points(&self) -> Option<&MultiPointProperty> {
        self.points.as_ref()
    }

    pub fn set_points(&mut self, points: MultiPointProperty) {
        self.points = Some(points);
    }

    pub fn set_points_opt(&mut self, points: Option<MultiPointProperty>) {
        self.points = points;
    }

    pub fn clear_points(&mut self) {
        self.points = None;
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

impl IterFeatures for PointCloud {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()).chain(self.abstract_point_cloud.iter_features()))
    }
}

impl ForEachFeatureMut for PointCloud {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_point_cloud.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for PointCloud {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_point_cloud.compute_envelope()
    }
}

impl ApplyTransform for PointCloud {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_point_cloud.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_point_cloud.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_point_cloud.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_point_cloud.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_point_cloud.apply_scale(scale);
    }
}
