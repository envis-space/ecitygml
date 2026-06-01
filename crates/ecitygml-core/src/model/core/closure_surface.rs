use crate::impl_abstract_thematic_surface_traits;
use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::core::{
    AbstractThematicSurface, AsAbstractFeatureMut, AsAbstractThematicSurface,
    AsAbstractThematicSurfaceMut,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct ClosureSurface {
    pub abstract_thematic_surface: AbstractThematicSurface,
}

impl ClosureSurface {
    pub fn new(abstract_thematic_surface: AbstractThematicSurface) -> Self {
        Self {
            abstract_thematic_surface,
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
}

impl AsAbstractThematicSurface for ClosureSurface {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        &self.abstract_thematic_surface
    }
}

impl AsAbstractThematicSurfaceMut for ClosureSurface {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        &mut self.abstract_thematic_surface
    }
}

impl_abstract_thematic_surface_traits!(ClosureSurface);

impl<'a> From<&'a ClosureSurface> for FeatureRef<'a> {
    fn from(item: &'a ClosureSurface) -> Self {
        FeatureRef::ClosureSurface(item)
    }
}

impl<'a> From<&'a mut ClosureSurface> for FeatureRefMut<'a> {
    fn from(item: &'a mut ClosureSurface) -> Self {
        FeatureRefMut::ClosureSurface(item)
    }
}
