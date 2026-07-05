use crate::model::construction::{
    AbstractConstructionSurface, AsAbstractConstructionSurface, AsAbstractConstructionSurfaceMut,
};
use crate::model::core::AsAbstractFeatureMut;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct RoofSurface {
    pub(crate) abstract_construction_surface: AbstractConstructionSurface,
}

impl RoofSurface {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_construction_surface(AbstractConstructionSurface::new(id))
    }

    pub fn from_abstract_construction_surface(
        abstract_construction_surface: AbstractConstructionSurface,
    ) -> Self {
        Self {
            abstract_construction_surface,
        }
    }
}
impl RoofSurface {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_construction_surface.iter_features())
    }
    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_construction_surface.for_each_feature_mut(f);
    }
    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_construction_surface.compute_envelope()
    }
    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }
    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_construction_surface.apply_transform(m);
    }
}

impl AsAbstractConstructionSurface for RoofSurface {
    fn abstract_construction_surface(&self) -> &AbstractConstructionSurface {
        &self.abstract_construction_surface
    }
}

impl AsAbstractConstructionSurfaceMut for RoofSurface {
    fn abstract_construction_surface_mut(&mut self) -> &mut AbstractConstructionSurface {
        &mut self.abstract_construction_surface
    }
}

crate::impl_abstract_construction_surface_traits!(RoofSurface);
crate::impl_abstract_construction_surface_mut_traits!(RoofSurface);
crate::impl_has_feature_type!(RoofSurface, RoofSurface);
