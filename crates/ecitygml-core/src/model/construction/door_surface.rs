use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::construction::{
    AbstractFillingSurface, AsAbstractFillingSurface, AsAbstractFillingSurfaceMut,
};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct DoorSurface {
    pub(crate) abstract_filling_surface: AbstractFillingSurface,
}

impl DoorSurface {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_filling_surface(AbstractFillingSurface::new(id))
    }

    pub fn from_abstract_filling_surface(abstract_filling_surface: AbstractFillingSurface) -> Self {
        Self {
            abstract_filling_surface,
        }
    }
}

impl AsAbstractFillingSurface for DoorSurface {
    fn abstract_filling_surface(&self) -> &AbstractFillingSurface {
        &self.abstract_filling_surface
    }
}

impl AsAbstractFillingSurfaceMut for DoorSurface {
    fn abstract_filling_surface_mut(&mut self) -> &mut AbstractFillingSurface {
        &mut self.abstract_filling_surface
    }
}

crate::impl_abstract_filling_surface_traits!(DoorSurface);
crate::impl_abstract_filling_surface_mut_traits!(DoorSurface);
crate::impl_has_feature_type!(DoorSurface, DoorSurface);

impl IterFeatures for DoorSurface {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()).chain(self.abstract_filling_surface.iter_features()))
    }
}

impl ForEachFeatureMut for DoorSurface {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_filling_surface.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for DoorSurface {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_filling_surface.compute_envelope()
    }
}

impl ApplyTransform for DoorSurface {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_filling_surface.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_filling_surface.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_filling_surface.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_filling_surface.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_filling_surface.apply_scale(scale);
    }
}
