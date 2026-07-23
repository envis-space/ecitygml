use crate::model::core::values::MimeTypeValue;
use crate::model::core::{AbstractAppearanceProperty, TransformationMatrix4x4};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::primitives::PointProperty;
use egml::model::geometry::{AbstractGeometryProperty, Envelope};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct ImplicitGeometry {
    object_id: Id,
    transformation_matrix: TransformationMatrix4x4,
    mime_type: Option<MimeTypeValue>,
    library_object: Option<String>,
    reference_point: PointProperty,
    relative_geometry: Option<AbstractGeometryProperty>,
    appearances: Vec<AbstractAppearanceProperty>,
}

impl ImplicitGeometry {
    pub fn new(
        object_id: Id,
        transformation_matrix: TransformationMatrix4x4,
        reference_point: PointProperty,
    ) -> Self {
        Self {
            object_id,
            transformation_matrix,
            mime_type: None,
            library_object: None,
            reference_point,
            relative_geometry: None,
            appearances: Vec::new(),
        }
    }

    pub fn object_id(&self) -> &Id {
        &self.object_id
    }

    pub fn set_object_id(&mut self, object_id: Id) {
        self.object_id = object_id;
    }

    pub fn transformation_matrix(&self) -> &TransformationMatrix4x4 {
        &self.transformation_matrix
    }

    pub fn set_transformation_matrix(&mut self, transformation_matrix: TransformationMatrix4x4) {
        self.transformation_matrix = transformation_matrix;
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

    pub fn library_object(&self) -> Option<&str> {
        self.library_object.as_deref()
    }

    pub fn set_library_object(&mut self, library_object: String) {
        self.library_object = Some(library_object);
    }

    pub fn set_library_object_opt(&mut self, library_object: Option<String>) {
        self.library_object = library_object;
    }

    pub fn clear_library_object(&mut self) {
        self.library_object = None;
    }

    pub fn reference_point(&self) -> &PointProperty {
        &self.reference_point
    }

    pub fn reference_point_mut(&mut self) -> &mut PointProperty {
        &mut self.reference_point
    }

    pub fn set_reference_point(&mut self, reference_point: PointProperty) {
        self.reference_point = reference_point;
    }

    pub fn relative_geometry(&self) -> Option<&AbstractGeometryProperty> {
        self.relative_geometry.as_ref()
    }

    pub fn relative_geometry_mut(&mut self) -> Option<&mut AbstractGeometryProperty> {
        self.relative_geometry.as_mut()
    }

    pub fn set_relative_geometry(&mut self, relative_geometry: AbstractGeometryProperty) {
        self.relative_geometry = Some(relative_geometry);
    }

    pub fn set_relative_geometry_opt(
        &mut self,
        relative_geometry: Option<AbstractGeometryProperty>,
    ) {
        self.relative_geometry = relative_geometry;
    }

    pub fn clear_relative_geometry(&mut self) {
        self.relative_geometry = None;
    }

    pub fn appearances(&self) -> &[AbstractAppearanceProperty] {
        &self.appearances
    }

    fn appearances_mut(&mut self) -> &mut Vec<AbstractAppearanceProperty> {
        &mut self.appearances
    }

    pub fn set_appearances(&mut self, appearances: Vec<AbstractAppearanceProperty>) {
        self.appearances = appearances;
    }

    fn push_appearance(&mut self, appearance: AbstractAppearanceProperty) {
        self.appearances.push(appearance);
    }

    fn extend_appearances(
        &mut self,
        appearances: impl IntoIterator<Item = AbstractAppearanceProperty>,
    ) {
        self.appearances.extend(appearances);
    }
}

impl ComputeEnvelope for ImplicitGeometry {
    fn compute_envelope(&self) -> Option<Envelope> {
        // TODO: compute envelope containing referenced geometry
        if let Some(point) = self.reference_point.object() {
            return point.compute_envelope();
        }

        None
    }
}

impl ApplyTransform for ImplicitGeometry {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        if let Some(point) = self.reference_point.object_mut() {
            point.apply_transform(m);
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        if let Some(point) = self.reference_point.object_mut() {
            point.apply_isometry(isometry);
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        if let Some(point) = self.reference_point.object_mut() {
            point.apply_translation(vector);
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        if let Some(point) = self.reference_point.object_mut() {
            point.apply_rotation(rotation);
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        if let Some(point) = self.reference_point.object_mut() {
            point.apply_scale(scale);
        }
    }
}
