use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::construction::{
    AbstractFillingSurface, AsAbstractFillingSurface, AsAbstractFillingSurfaceMut,
};
use crate::model::core::AsAbstractFeatureMut;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct WindowSurface {
    pub abstract_filling_surface: AbstractFillingSurface,
}

impl WindowSurface {
    pub fn new(abstract_filling_surface: AbstractFillingSurface) -> Self {
        Self {
            abstract_filling_surface,
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_filling_surface.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_filling_surface.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_filling_surface.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_filling_surface.apply_transform(m);
    }
}

impl AsAbstractFillingSurface for WindowSurface {
    fn abstract_filling_surface(&self) -> &AbstractFillingSurface {
        &self.abstract_filling_surface
    }
}

impl AsAbstractFillingSurfaceMut for WindowSurface {
    fn abstract_filling_surface_mut(&mut self) -> &mut AbstractFillingSurface {
        &mut self.abstract_filling_surface
    }
}

crate::impl_abstract_filling_surface_traits!(WindowSurface);

impl<'a> From<&'a WindowSurface> for FeatureRef<'a> {
    fn from(item: &'a WindowSurface) -> Self {
        FeatureRef::WindowSurface(item)
    }
}

impl<'a> From<&'a mut WindowSurface> for FeatureRefMut<'a> {
    fn from(item: &'a mut WindowSurface) -> Self {
        FeatureRefMut::WindowSurface(item)
    }
}
