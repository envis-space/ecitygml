use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::core::{
    AbstractThematicSurface, AsAbstractFeatureMut, AsAbstractThematicSurface,
    AsAbstractThematicSurfaceMut,
};
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct TrafficArea {
    pub(crate) abstract_thematic_surface: AbstractThematicSurface,
    pub(crate) class: Option<Code>,
    pub(crate) functions: Vec<Code>,
    pub(crate) usages: Vec<Code>,
    pub(crate) surface_material: Option<Code>,
}

impl TrafficArea {
    pub fn new(abstract_thematic_surface: AbstractThematicSurface) -> Self {
        Self {
            abstract_thematic_surface,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
            surface_material: None,
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_thematic_surface.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_thematic_surface.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_thematic_surface.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_thematic_surface.apply_transform(m);
    }

    pub fn class(&self) -> &Option<Code> {
        &self.class
    }

    pub fn set_class(&mut self, class: Option<Code>) {
        self.class = class;
    }

    pub fn functions(&self) -> &Vec<Code> {
        &self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<Code>) {
        self.functions = functions;
    }

    pub fn usages(&self) -> &Vec<Code> {
        &self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<Code>) {
        self.usages = usages;
    }

    pub fn surface_material(&self) -> Option<&Code> {
        self.surface_material.as_ref()
    }

    pub fn set_surface_material(&mut self, surface_material: Option<Code>) {
        self.surface_material = surface_material;
    }
}

impl AsAbstractThematicSurface for TrafficArea {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        &self.abstract_thematic_surface
    }
}

impl AsAbstractThematicSurfaceMut for TrafficArea {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        &mut self.abstract_thematic_surface
    }
}

crate::impl_abstract_thematic_surface_traits!(TrafficArea);

impl<'a> From<&'a TrafficArea> for FeatureRef<'a> {
    fn from(item: &'a TrafficArea) -> Self {
        FeatureRef::TrafficArea(item)
    }
}

impl<'a> From<&'a mut TrafficArea> for FeatureRefMut<'a> {
    fn from(item: &'a mut TrafficArea) -> Self {
        FeatureRefMut::TrafficArea(item)
    }
}
