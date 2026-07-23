use crate::model::appearance::basic_types::Color;
use crate::model::appearance::{
    AbstractSurfaceData, AsAbstractSurfaceData, AsAbstractSurfaceDataMut, GeometryReference,
};
use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::basic_types::DoubleBetween0And1;
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct X3DMaterial {
    pub(crate) abstract_surface_data: AbstractSurfaceData,
    ambient_intensity: Option<DoubleBetween0And1>,
    diffuse_color: Option<Color>,
    emissive_color: Option<Color>,
    specular_color: Option<Color>,
    shininess: Option<DoubleBetween0And1>,
    transparency: Option<DoubleBetween0And1>,
    is_smooth: Option<bool>,
    targets: Vec<GeometryReference>,
}

impl X3DMaterial {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_surface_data(AbstractSurfaceData::new(id))
    }

    pub fn from_abstract_surface_data(abstract_surface_data: AbstractSurfaceData) -> Self {
        Self {
            abstract_surface_data,
            ambient_intensity: None,
            diffuse_color: None,
            emissive_color: None,
            specular_color: None,
            shininess: None,
            transparency: None,
            is_smooth: None,
            targets: Vec::new(),
        }
    }

    pub fn ambient_intensity(&self) -> Option<DoubleBetween0And1> {
        self.ambient_intensity
    }

    pub fn set_ambient_intensity(&mut self, ambient_intensity: DoubleBetween0And1) {
        self.ambient_intensity = Some(ambient_intensity);
    }

    pub fn set_ambient_intensity_opt(&mut self, ambient_intensity: Option<DoubleBetween0And1>) {
        self.ambient_intensity = ambient_intensity;
    }

    pub fn clear_ambient_intensity(&mut self) {
        self.ambient_intensity = None;
    }

    pub fn diffuse_color(&self) -> Option<Color> {
        self.diffuse_color
    }

    pub fn set_diffuse_color(&mut self, diffuse_color: Color) {
        self.diffuse_color = Some(diffuse_color);
    }

    pub fn set_diffuse_color_opt(&mut self, diffuse_color: Option<Color>) {
        self.diffuse_color = diffuse_color;
    }

    pub fn clear_diffuse_color(&mut self) {
        self.diffuse_color = None;
    }

    pub fn emissive_color(&self) -> Option<Color> {
        self.emissive_color
    }

    pub fn set_emissive_color(&mut self, emissive_color: Color) {
        self.emissive_color = Some(emissive_color);
    }

    pub fn set_emissive_color_opt(&mut self, emissive_color: Option<Color>) {
        self.emissive_color = emissive_color;
    }

    pub fn clear_emissive_color(&mut self) {
        self.emissive_color = None;
    }

    pub fn specular_color(&self) -> Option<Color> {
        self.specular_color
    }

    pub fn set_specular_color(&mut self, specular_color: Color) {
        self.specular_color = Some(specular_color);
    }

    pub fn set_specular_color_opt(&mut self, specular_color: Option<Color>) {
        self.specular_color = specular_color;
    }

    pub fn clear_specular_color(&mut self) {
        self.specular_color = None;
    }

    pub fn shininess(&self) -> Option<DoubleBetween0And1> {
        self.shininess
    }

    pub fn set_shininess(&mut self, shininess: DoubleBetween0And1) {
        self.shininess = Some(shininess);
    }

    pub fn set_shininess_opt(&mut self, shininess: Option<DoubleBetween0And1>) {
        self.shininess = shininess;
    }

    pub fn clear_shininess(&mut self) {
        self.shininess = None;
    }

    pub fn transparency(&self) -> Option<DoubleBetween0And1> {
        self.transparency
    }

    pub fn set_transparency(&mut self, transparency: DoubleBetween0And1) {
        self.transparency = Some(transparency);
    }

    pub fn set_transparency_opt(&mut self, transparency: Option<DoubleBetween0And1>) {
        self.transparency = transparency;
    }

    pub fn clear_transparency(&mut self) {
        self.transparency = None;
    }

    pub fn is_smooth(&self) -> Option<bool> {
        self.is_smooth
    }

    pub fn set_is_smooth(&mut self, is_smooth: Option<bool>) {
        self.is_smooth = is_smooth;
    }

    pub fn targets(&self) -> &[GeometryReference] {
        &self.targets
    }

    pub fn targets_mut(&mut self) -> &mut [GeometryReference] {
        &mut self.targets
    }

    pub fn set_targets(&mut self, targets: Vec<GeometryReference>) {
        self.targets = targets;
    }

    pub fn push_target(&mut self, target: GeometryReference) {
        self.targets.push(target);
    }

    pub fn extend_targets(&mut self, targets: impl IntoIterator<Item = GeometryReference>) {
        self.targets.extend(targets);
    }
}

impl AsAbstractSurfaceData for X3DMaterial {
    fn abstract_surface_data(&self) -> &AbstractSurfaceData {
        &self.abstract_surface_data
    }
}

impl AsAbstractSurfaceDataMut for X3DMaterial {
    fn abstract_surface_data_mut(&mut self) -> &mut AbstractSurfaceData {
        &mut self.abstract_surface_data
    }
}

crate::impl_abstract_surface_data_traits!(X3DMaterial);
crate::impl_abstract_surface_data_mut_traits!(X3DMaterial);
crate::impl_has_feature_type!(X3DMaterial, X3DMaterial);

impl IterFeatures for X3DMaterial {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()).chain(self.abstract_surface_data.iter_features()))
    }
}

impl ForEachFeatureMut for X3DMaterial {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_surface_data.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for X3DMaterial {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_surface_data.compute_envelope()
    }
}

impl ApplyTransform for X3DMaterial {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_surface_data.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_surface_data.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_surface_data.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_surface_data.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_surface_data.apply_scale(scale);
    }
}
