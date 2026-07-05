use crate::model::appearance::basic_types::Color;
use crate::model::appearance::{
    AbstractSurfaceData, AsAbstractSurfaceData, AsAbstractSurfaceDataMut, GeometryReference,
};
use crate::model::core::AsAbstractFeatureMut;
use crate::model::core::basic_types::DoubleBetween0And1;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

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

    pub fn set_ambient_intensity(&mut self, ambient_intensity: Option<DoubleBetween0And1>) {
        self.ambient_intensity = ambient_intensity;
    }

    pub fn diffuse_color(&self) -> Option<Color> {
        self.diffuse_color
    }

    pub fn set_diffuse_color(&mut self, diffuse_color: Option<Color>) {
        self.diffuse_color = diffuse_color;
    }

    pub fn emissive_color(&self) -> Option<Color> {
        self.emissive_color
    }

    pub fn set_emissive_color(&mut self, emissive_color: Option<Color>) {
        self.emissive_color = emissive_color;
    }

    pub fn specular_color(&self) -> Option<Color> {
        self.specular_color
    }

    pub fn set_specular_color(&mut self, specular_color: Option<Color>) {
        self.specular_color = specular_color;
    }

    pub fn shininess(&self) -> Option<DoubleBetween0And1> {
        self.shininess
    }

    pub fn set_shininess(&mut self, shininess: Option<DoubleBetween0And1>) {
        self.shininess = shininess;
    }

    pub fn transparency(&self) -> Option<DoubleBetween0And1> {
        self.transparency
    }

    pub fn set_transparency(&mut self, transparency: Option<DoubleBetween0And1>) {
        self.transparency = transparency;
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

impl X3DMaterial {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_surface_data.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_surface_data.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_surface_data.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_surface_data.apply_transform(m);
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
