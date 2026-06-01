use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::core::{
    AbstractThematicSurface, AsAbstractFeatureMut, AsAbstractThematicSurface,
    AsAbstractThematicSurfaceMut,
};
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct Marking {
    pub(crate) abstract_thematic_surface: AbstractThematicSurface,
    pub(crate) class: Option<Code>,
}

impl Marking {
    pub fn new(abstract_thematic_surface: AbstractThematicSurface) -> Self {
        Self {
            abstract_thematic_surface,
            class: None,
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
}

impl AsAbstractThematicSurface for Marking {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        &self.abstract_thematic_surface
    }
}

impl AsAbstractThematicSurfaceMut for Marking {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        &mut self.abstract_thematic_surface
    }
}

crate::impl_abstract_thematic_surface_traits!(Marking);

impl<'a> From<&'a Marking> for FeatureRef<'a> {
    fn from(item: &'a Marking) -> Self {
        FeatureRef::Marking(item)
    }
}

impl<'a> From<&'a mut Marking> for FeatureRefMut<'a> {
    fn from(item: &'a mut Marking) -> Self {
        FeatureRefMut::Marking(item)
    }
}
